//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1170/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1170(t1092: f64, t19663: f64, t13271: f64, t13278: f64, t13302: f64, t13303: f64, t13305: f64, t13308: f64, t13312: f64, t19628: f64, t19633: f64, t19636: f64, t19640: f64, t19642: f64, t19645: f64, t19648: f64, t19651: f64, t19658: f64, t19661: f64, t9552: f64) -> (f64, f64) {
    let t19664 = t1092 * t19663;
    let t19670 = -0.33163888888888888888e-2_f64 * t19628 - 0.33163888888888888888e-2_f64 * t19633 + 0.16581944444444444444e-2_f64 * t19636 + 0.33163888888888888888e-2_f64 * t19640 - 0.33163888888888888888e-2_f64 * t19642 + 0.13265555555555555555e-1_f64 * t19645 + 0.33163888888888888888e-2_f64 * t19648 + 0.16581944444444444444e-2_f64 * t19651 + 0.16581944444444444444e-2_f64 * t19658 - 0.49745833333333333332e-2_f64 * t19661 + t13271 + 0.99491666666666666664e-2_f64 * t19664 - 0.36848765432098765431e-3_f64 * t9552 - t13278 + t13302 + 0.11054629629629629629e-2_f64 * t13303 + 0.88437037037037037035e-2_f64 * t13305 + t13308 - 0.58958024691358024688e-2_f64 * t13312;
    (t19664, t19670)
}
