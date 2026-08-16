//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 723/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk723(t10513: f64, t10515: f64, t10517: f64, t10525: f64, t10527: f64, t10530: f64, t10532: f64, t10537: f64, t11190: f64, t11197: f64, t11201: f64, t11204: f64, t11209: f64, t11211: f64, t11216: f64, t11222: f64, t11231: f64, t11233: f64, t1693: f64, t1792: f64, t4830: f64, t5044: f64) -> f64 {
    let t11235 = -0.74618749999999999998e-2_f64 * t10513 + 0.33163888888888888887e-2_f64 * t10515 - 0.16581944444444444444e-2_f64 * t10517 + 0.16581944444444444444e-2_f64 * t10525 + 0.66327777777777777776e-2_f64 * t10527 - 0.16581944444444444444e-2_f64 * t10530 - 0.49745833333333333332e-2_f64 * t10532 - 0.49745833333333333332e-2_f64 * t10537 - 0.193e0_f64 * t1693 * t11190 - 0.579e0_f64 * t4830 * t5044 - 0.43134342e-1_f64 * t11197 * t11201 - 0.579e0_f64 * t11204 * t1792 + 0.16581944444444444444e-2_f64 * t11209 - 0.11054629629629629629e-2_f64 * t11211 + 0.73697530864197530862e-3_f64 * t11216 + 0.55273148148148148145e-2_f64 * t11222 - 0.1492375e-1_f64 * t11231 - 0.11054629629629629629e-2_f64 * t11233;
    t11235
}
