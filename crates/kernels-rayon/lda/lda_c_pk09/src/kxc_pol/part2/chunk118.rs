//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 118/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk118(t339: f64, t340: f64, t294: f64, t307: f64, t311: f64, t319: f64, t328: f64, t335: f64, t305: f64, t323: f64) -> (f64, f64, f64) {
    let t341 = t339 * t340;
    let t343 = -t307 * t311 / 6.0_f64 - t319 * t311 / 6.0_f64 + t328 * t311 / 6.0_f64 - 0.10237773105191754_f64 * t294 + 1.0150830754383913_f64 + 0.14975624337724558_f64 * t335 + 0.018501446123012983_f64 * t341;
    let t347 = t323 * t305;
    (t341, t343, t347)
}
