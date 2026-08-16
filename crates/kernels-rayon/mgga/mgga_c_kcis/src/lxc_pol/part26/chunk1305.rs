//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1305/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1305(t21935: f64, t4153: f64, t7923: f64, t21939: f64, t101875: f64, t101950: f64, t102005: f64, t102068: f64, t102311: f64, t102313: f64, t102318: f64, t102328: f64, t102334: f64, t27595: f64, t7968: f64, t7981: f64) -> (f64, f64, f64) {
    let t102337 = t4153 * t7923 * t21935;
    let t102340 = t4153 * t7923 * t21939;
    let t102342 = 0.77382407407407407407e-3_f64 * t102311 - 0.15476481481481481481e-2_f64 * t102313 - 0.11326774691358024691e-2_f64 * t101950 * t7981 - 0.51485339506172839507e-4_f64 * t102318 - 0.46377350260416666667e-4_f64 * t7968 * t102068 - 0.13913205078125e-3_f64 * t7968 * t102005 - 0.92835860883789062501e-5_f64 * t27595 * t102005 + 0.92835860883789062501e-5_f64 * t27595 * t102328 + 0.557015165302734375e-4_f64 * t27595 * t101875 - 0.19345601851851851852e-2_f64 * t102334 + 0.12897067901234567901e-2_f64 * t102337 - 0.11607361111111111111e-1_f64 * t102340;
    (t102337, t102340, t102342)
}
