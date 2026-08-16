//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1373/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1373(t10241: f64, t5907: f64, t661: f64, t1509: f64, t2: f64, t580: f64, t2357: f64, t5911: f64, t21850: f64, t108: f64, t105: f64, t13475: f64, t13496: f64, t1507: f64, t21836: f64, t21840: f64, t21846: f64, t21851: f64, t4280: f64, t4284: f64, t5896: f64, t5899: f64, t5902: f64, t656: f64, t662: f64, t97: f64) -> f64 {
    let t21860 = t10241 * t5907;
    let t21861 = t21860 * t661;
    let t21864 = t1509 * t2;
    let t21865 = t21864 * t580;
    let t21868 = t2357 * t5911;
    let t21869 = t21868 * t661;
    let t21872 = -t21850;
    let t21873 = t108 * t21872;
    let t21876 = -50.0_f64 / 27.0_f64 * t656 * t5896 - 10.0_f64 / 27.0_f64 * t97 * t21836 + 20.0_f64 / 9.0_f64 * t13475 * t21840 - 25.0_f64 / 9.0_f64 * t656 * t5899 + 10.0_f64 / 9.0_f64 * t97 * t21846 + 5.0_f64 / 3.0_f64 * t97 * t21851 + 200.0_f64 / 27.0_f64 * t5902 * t662 - 100.0_f64 / 27.0_f64 * t1507 * t4280 + 50.0_f64 / 9.0_f64 * t1507 * t4284 - 10.0_f64 / 27.0_f64 * t105 * t21861 - 20.0_f64 / 9.0_f64 * t13496 * t21865 + 10.0_f64 / 9.0_f64 * t105 * t21869 + 5.0_f64 / 3.0_f64 * t105 * t21873;
    t21876
}
