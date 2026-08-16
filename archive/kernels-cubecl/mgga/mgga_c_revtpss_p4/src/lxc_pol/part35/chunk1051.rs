//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1051/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1051<F: Float>(t239: F, t72: F, t1927: F, t1923: F, t3920: F, t7496: F, t2098: F, t2453: F, t3908: F, t2097: F, t25937: F, t7282: F) -> (F, F, F, F, F, F, F, F) {
    let t26204 = t239 * t72;
    let t26205 = t26204 * t1927;
    let t26207 = F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t1923 * t26205;
    let t26238 = F::cast_from(0.13009920719177044025e-1_f64) * t7496 * t3920;
    let t26249 = t2453 * t2098;
    let t26251 = F::cast_from(0.11565819519348392139e-2_f64) * t26249 * t3908;
    let t26260 = t25937 * t2097;
    let t26261 = t7282 * t26260;
    (t26204, t26205, t26207, t26238, t26249, t26251, t26260, t26261)
}
