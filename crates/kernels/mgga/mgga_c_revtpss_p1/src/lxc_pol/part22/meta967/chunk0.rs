//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3231/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3231<F: Float>(t14622: F, t18259: F, t18281: F, t189: F, t4401: F, t606: F, t190: F, t2611: F, t60717: F, t18555: F, t2619: F, t13396: F, t14330: F, t4402: F) -> (F, F, F, F, F) {
    let t61265 = F::cast_from(24.0_f64) * t18259 * t14622;
    let t61266 = t189 * t18281;
    let t61269 = F::cast_from(24.0_f64) * t4401 * t61266 * t606;
    let t61274 = F::cast_from(24.0_f64) * t2611 * t190 * t60717;
    let t61282 = t18555 * t2619;
    let t61283 = F::cast_from(0.24415263074675393405e-3_f64) * t61282;
    let t61286 = F::cast_from(96.0_f64) * t14330 * t4402 * t13396;
    (t61265, t61269, t61274, t61283, t61286)
}
