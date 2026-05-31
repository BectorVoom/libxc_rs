//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 868/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk868<F: Float>(t2047: F, t25163: F, t6963: F, t7349: F, t10301: F, t7342: F, t6954: F, t239: F, t72: F, t1927: F, t1923: F, t122: F, t2097: F) -> (F, F, F, F, F, F) {
    let t26182 = t2047 * t25163;
    let t26185 = t6963 * t7349;
    let t26187 = t10301 * t7342;
    let t26190 = t6954 * t7349;
    let t26204 = t239 * t72;
    let t26205 = t26204 * t1927;
    let t26207 = F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t1923 * t26205;
    let t26230 = t2097 * t72 * t122;
    (t26182, t26185, t26187, t26190, t26207, t26230)
}
