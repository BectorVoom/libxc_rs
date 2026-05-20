//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1691/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1691<F: Float>(t1923: F, t26205: F, t122: F, t2097: F, t72: F, t25900: F, t25904: F, t3916: F, t25895: F, t3920: F, t7496: F, t2098: F, t2453: F) -> (F, F, F, F, F, F, F, F) {
    let t26207 = F::new(88.0) / F::new(27.0) * t1923 * t26205;
    let t26230 = t2097 * t72 * t122;
    let t26231 = t26230 * t25900;
    let t26232 = t25904 * t26231;
    let t26234 = t26230 * t3916;
    let t26235 = t25895 * t26234;
    let t26238 = F::cast_from(0.13009920719177044025e-1_f64) * t7496 * t3920;
    let t26249 = t2453 * t2098;
    (t26207, t26230, t26231, t26232, t26234, t26235, t26238, t26249)
}
