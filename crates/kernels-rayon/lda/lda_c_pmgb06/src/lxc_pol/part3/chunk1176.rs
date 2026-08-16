//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1176/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1176(t161: f64, t489: f64, t4940: f64, t10412: f64, t1385: f64, t1420: f64, t1604: f64, t1629: f64, t1848: f64, t1868: f64, t1893: f64, t1898: f64, t2010: f64, t2948: f64, t3040: f64, t3177: f64, t439: f64, t486: f64, t4945: f64, t5168: f64, t5226: f64, t5290: f64, t5291: f64, t5294: f64, t5295: f64, t831: f64) -> f64 {
    let t14024 = t161 * t489 * t4940;
    let t14053 = -2.0_f64 / 15.0_f64 * t14024 + t1848 * t1604 / 5.0_f64 + t831 * t3040 / 5.0_f64 - t486 * t4945 / 10.0_f64 - 2.0_f64 / 15.0_f64 * t2010 * t1385 * t1868 * t1629 - 2.0_f64 / 15.0_f64 * t3177 * t1898 - 4.0_f64 / 15.0_f64 * t1420 * t5226 - t1420 * t5291 / 15.0_f64 - 4.0_f64 / 15.0_f64 * t5168 * t5295 - t439 * t10412 * t1893 / 15.0_f64 - t439 * t2948 * t5290 / 15.0_f64 - 4.0_f64 / 15.0_f64 * t2010 * t2948 * t5294;
    t14053
}
