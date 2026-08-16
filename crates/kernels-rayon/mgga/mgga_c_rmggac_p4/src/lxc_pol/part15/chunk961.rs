//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 961/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk961(t38397: f64, t8571: f64, t1981: f64, t629: f64, t676: f64, t8512: f64, t1971: f64, t495: f64, t515: f64, t8517: f64, t9843: f64, t42054: f64) -> (f64, f64, f64, f64) {
    let t45938 = t8571 * t38397;
    let t45942 = t8512 * t1981 * t676 * t629;
    let t45947 = t8517 * t1971 * t515 * t9843 * t495;
    let t45949 = t8571 * t42054;
    (t45938, t45942, t45947, t45949)
}
