//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 617/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk617<F: Float>(t10503: F, t1790: F, t4581: F, t4804: F, t1869: F, t10429: F, t10434: F, t10439: F, t10445: F, t10453: F, t10456: F, t10467: F, t10469: F, t10474: F, t10477: F, t10482: F, t10484: F, t10491: F, t10495: F, t10497: F, t10502: F, t1693: F) -> (F, F, F) {
    let t10504 = t10503 * t1790;
    let t10507 = t4581 * t4804;
    let t10508 = t1869 * t10507;
    let t10510 = -0.66327777777777777776e-2 * t10429 - 0.49745833333333333332e-2 * t10434 - 0.16581944444444444444e-2 * t10439 + 0.99491666666666666664e-2 * t10445 + 0.16581944444444444444e-2 * t10453 + 0.8290972222222222222e-2 * t10456 + 0.73697530864197530861e-2 * t10467 - 0.66327777777777777776e-2 * t10469 - 0.11054629629629629629e-2 * t10474 - 0.99491666666666666664e-2 * t10477 + 0.16581944444444444444e-2 * t10482 - 0.66327777777777777775e-2 * t10484 - 0.16581944444444444444e-1 * t10491 + 0.55273148148148148145e-2 * t10495 + 0.33163888888888888887e-2 * t10497 + t10502 + 0.579e0 * t1693 * t10504 + 0.49745833333333333332e-2 * t10508;
    (t10504, t10508, t10510)
}
