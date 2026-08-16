//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 662/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk662<F: Float>(t10500: F, t721: F, t4826: F, t5043: F, t1790: F, t4581: F, t4804: F, t1869: F, t10429: F, t10434: F, t10439: F, t10445: F, t10453: F, t10456: F, t10467: F, t10469: F, t10474: F, t10477: F, t10482: F, t10484: F, t10491: F, t10495: F, t10497: F, t1693: F) -> (F, F, F, F) {
    let t10501 = t10500 * t721;
    let t10502 = F::cast_from(0.73697530864197530862e-3_f64) * t10501;
    let t10503 = t5043 * t4826;
    let t10504 = t10503 * t1790;
    let t10507 = t4581 * t4804;
    let t10508 = t1869 * t10507;
    let t10510 = -F::cast_from(0.66327777777777777776e-2_f64) * t10429 - F::cast_from(0.49745833333333333332e-2_f64) * t10434 - F::cast_from(0.16581944444444444444e-2_f64) * t10439 + F::cast_from(0.99491666666666666664e-2_f64) * t10445 + F::cast_from(0.16581944444444444444e-2_f64) * t10453 + F::cast_from(0.8290972222222222222e-2_f64) * t10456 + F::cast_from(0.73697530864197530861e-2_f64) * t10467 - F::cast_from(0.66327777777777777776e-2_f64) * t10469 - F::cast_from(0.11054629629629629629e-2_f64) * t10474 - F::cast_from(0.99491666666666666664e-2_f64) * t10477 + F::cast_from(0.16581944444444444444e-2_f64) * t10482 - F::cast_from(0.66327777777777777775e-2_f64) * t10484 - F::cast_from(0.16581944444444444444e-1_f64) * t10491 + F::cast_from(0.55273148148148148145e-2_f64) * t10495 + F::cast_from(0.33163888888888888887e-2_f64) * t10497 + t10502 + F::cast_from(0.579e0_f64) * t1693 * t10504 + F::cast_from(0.49745833333333333332e-2_f64) * t10508;
    (t10501, t10504, t10508, t10510)
}
