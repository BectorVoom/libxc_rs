//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 662/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk662(t10500: f64, t721: f64, t4826: f64, t5043: f64, t1790: f64, t4581: f64, t4804: f64, t1869: f64, t10429: f64, t10434: f64, t10439: f64, t10445: f64, t10453: f64, t10456: f64, t10467: f64, t10469: f64, t10474: f64, t10477: f64, t10482: f64, t10484: f64, t10491: f64, t10495: f64, t10497: f64, t1693: f64) -> (f64, f64, f64, f64) {
    let t10501 = t10500 * t721;
    let t10502 = 0.73697530864197530862e-3_f64 * t10501;
    let t10503 = t5043 * t4826;
    let t10504 = t10503 * t1790;
    let t10507 = t4581 * t4804;
    let t10508 = t1869 * t10507;
    let t10510 = -0.66327777777777777776e-2_f64 * t10429 - 0.49745833333333333332e-2_f64 * t10434 - 0.16581944444444444444e-2_f64 * t10439 + 0.99491666666666666664e-2_f64 * t10445 + 0.16581944444444444444e-2_f64 * t10453 + 0.8290972222222222222e-2_f64 * t10456 + 0.73697530864197530861e-2_f64 * t10467 - 0.66327777777777777776e-2_f64 * t10469 - 0.11054629629629629629e-2_f64 * t10474 - 0.99491666666666666664e-2_f64 * t10477 + 0.16581944444444444444e-2_f64 * t10482 - 0.66327777777777777775e-2_f64 * t10484 - 0.16581944444444444444e-1_f64 * t10491 + 0.55273148148148148145e-2_f64 * t10495 + 0.33163888888888888887e-2_f64 * t10497 + t10502 + 0.579e0_f64 * t1693 * t10504 + 0.49745833333333333332e-2_f64 * t10508;
    (t10501, t10504, t10508, t10510)
}
