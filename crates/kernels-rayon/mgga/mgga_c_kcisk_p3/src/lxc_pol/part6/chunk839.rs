//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 839/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk839(t10502: f64, t16640: f64, t16658: f64, t17078: f64, t22353: f64, t22355: f64, t28244: f64, t28250: f64, t28253: f64, t28259: f64, t28262: f64, t28271: f64, t4823: f64, t8852: f64) -> f64 {
    let t28273 = 0.55273148148148148145e-2_f64 * t28244 + 0.55273148148148148145e-2_f64 * t22353 + 0.33163888888888888887e-2_f64 * t22355 + 0.99491666666666666664e-2_f64 * t28250 + 0.8290972222222222222e-2_f64 * t28253 + t10502 - 0.16581944444444444444e-2_f64 * t16640 + 0.99491666666666666664e-2_f64 * t28259 - 0.223494e0_f64 * t4823 * t28262 + 0.223494e0_f64 * t17078 * t8852 - 0.11054629629629629629e-2_f64 * t16658 - 0.49745833333333333332e-2_f64 * t28271;
    t28273
}
