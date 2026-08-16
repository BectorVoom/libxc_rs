//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 632/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk632(t236: f64, t8829: f64, t3352: f64, t7230: f64, t333: f64, t615: f64, t511: f64, t1971: f64, t352: f64, t515: f64, t2320: f64, t7717: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8830 = t236 * t8829;
    let t8831 = t3352 * t8830;
    let t8832 = t7230 * t8831;
    let t8834 = t615 * t333;
    let t8835 = t511 * t8834;
    let t8836 = t1971 * t8835;
    let t8837 = t7230 * t8836;
    let t8842 = t515 * t615 * t352;
    let t8843 = t1971 * t8842;
    let t8844 = t7230 * t8843;
    let t8846 = t7717 * t2320;
    (t8831, t8832, t8836, t8837, t8843, t8844, t8846)
}
