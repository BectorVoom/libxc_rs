//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 787/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk787<F: Float>(t122: F, t2097: F, t72: F, t25900: F, t25904: F, t3916: F, t25895: F, t3920: F, t7496: F, t2098: F, t2453: F, t3908: F, t7507: F, t786: F, t1364: F, t25937: F) -> (F, F, F, F, F, F, F, F) {
    let t26230 = t2097 * t72 * t122;
    let t26231 = t26230 * t25900;
    let t26232 = t25904 * t26231;
    let t26234 = t26230 * t3916;
    let t26235 = t25895 * t26234;
    let t26238 = 0.13009920719177044025e-1 * t7496 * t3920;
    let t26249 = t2453 * t2098;
    let t26251 = 0.11565819519348392139e-2 * t26249 * t3908;
    let t26252 = t786 * t7507;
    let t26253 = t26252 * t1364;
    let t26260 = t25937 * t2097;
    (t26231, t26232, t26234, t26235, t26238, t26251, t26253, t26260)
}
