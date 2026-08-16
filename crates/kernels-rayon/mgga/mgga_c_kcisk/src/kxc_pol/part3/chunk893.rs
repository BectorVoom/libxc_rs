//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 893/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk893(t13361: f64, t1411: f64, t13318: f64, t13323: f64, t13325: f64, t13334: f64, t13338: f64, t13342: f64, t13344: f64, t13347: f64, t13351: f64, t13355: f64, t13359: f64) -> (f64, f64) {
    let t13362 = t1411 * t13361;
    let t13364 = 0.33163888888888888887e-2_f64 * t13318 - 0.49745833333333333332e-2_f64 * t13323 - 0.11054629629629629629e-2_f64 * t13325 - 0.1492375e-1_f64 * t13334 - 0.39796666666666666665e-1_f64 * t13338 + 0.99491666666666666664e-2_f64 * t13342 - 0.66327777777777777775e-2_f64 * t13344 - 0.99491666666666666664e-2_f64 * t13347 - 0.13265555555555555555e-1_f64 * t13351 - 0.22109259259259259258e-1_f64 * t13355 - 0.16581944444444444444e-1_f64 * t13359 - 0.99491666666666666664e-2_f64 * t13362;
    (t13362, t13364)
}
