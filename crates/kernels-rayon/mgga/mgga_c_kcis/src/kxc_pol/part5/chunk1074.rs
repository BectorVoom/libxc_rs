//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1074/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1074(t5412: f64, t5400: f64, t6262: f64, t4544: f64, t4528: f64, t13034: f64, t13043: f64, t13044: f64, t18413: f64, t18414: f64, t18415: f64, t18416: f64, t18417: f64, t18418: f64, t18419: f64, t6300: f64, t6886: f64, t6890: f64, t6899: f64, t6902: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18423 = t5412 / 8.0_f64;
    let t18424 = t5400 / 8.0_f64;
    let t18425 = t6262 / 8.0_f64;
    let t18426 = t4544 / 8.0_f64;
    let t18427 = 2.0_f64 * t4528;
    let t18429 = -t18413 - t6899 - t6890 - t18414 - t6886 + t18415 - t6300 - t13034 - t18416 - t6902 + t18417 + t18418 - t13044 + t13043 + t18419;
    (t18423, t18424, t18425, t18426, t18427, t18429)
}
