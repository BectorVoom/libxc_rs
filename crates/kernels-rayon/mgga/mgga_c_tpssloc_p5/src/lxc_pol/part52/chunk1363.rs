//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1363/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1363(t1395: f64, t1458: f64, t8319: f64, t1437: f64, t1862: f64, t645: f64, t8308: f64, t113875: f64, t641: f64, t1409: f64, t83817: f64, t31682: f64, t3966: f64) -> (f64, f64, f64, f64, f64) {
    let t120849 = t1395 * t1458;
    let t120851 = 27.0_f64 * t120849 * t8319;
    let t121022 = t1862 * t1437;
    let t121024 = t8308 * t121022 * t645;
    let t121032 = t113875 * t121022 * t641;
    let t121040 = t8308 * t83817 * t1409;
    let t121044 = t8308 * t31682 * t3966;
    (t120851, t121024, t121032, t121040, t121044)
}
