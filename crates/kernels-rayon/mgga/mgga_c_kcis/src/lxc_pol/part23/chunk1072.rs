//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1072/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1072(t1610: f64, t1615: f64, t27614: f64, t6176: f64, t27381: f64, t27385: f64, t27569: f64, t27583: f64, t27586: f64, t27592: f64, t27595: f64, t27598: f64, t27602: f64, t27604: f64, t27607: f64, t7971: f64, t7978: f64, t7986: f64) -> (f64, f64, f64, f64) {
    let t27615 = t1610 * t1615;
    let t27616 = t27614 * t27615;
    let t27617 = t6176 * t27616;
    let t27620 = 0.23168402777777777778e-3_f64 * t27583 * t27586 + 0.23168402777777777778e-3_f64 * t27583 * t27569 - 0.7722800925925925926e-4_f64 * t27592 - 0.92835860883789062501e-5_f64 * t27595 * t27598 + 0.23168402777777777778e-3_f64 * t27602 + 0.23168402777777777778e-3_f64 * t27604 + 0.69505208333333333334e-3_f64 * t27607 * t7986 + 0.69505208333333333334e-3_f64 * t27607 * t7971 + 0.11607361111111111111e-2_f64 * t27381 + 0.19345601851851851852e-2_f64 * t27385 - 0.69505208333333333334e-3_f64 * t7978 * t27617;
    (t27615, t27616, t27617, t27620)
}
