//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 774/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk774(t312: f64, t4242: f64, t4245: f64, t4249: f64, t4296: f64, t4301: f64, t4304: f64, t4307: f64, t4318: f64, t4322: f64, t4324: f64, t4325: f64, t5893: f64, t5896: f64, t5901: f64, t61: f64, t7124: f64, t7149: f64, t7170: f64, t7236: f64) -> f64 {
    let t7243 = -t5893 - 0.02394846802050922_f64 * t5896 - 3.64371538634302e-05_f64 * t5901 + (t7124 + t7149) * t312 + (t7170 + t7236) * t61 + t4242 - 1.82185769317151e-05_f64 * t4245 - t4249 - t4296 - t4301 + 0.039914113367515366_f64 * t4304 + t4307 - 0.01197423401025461_f64 * t4318 + t4322 - t4324 - 0.05321881782335382_f64 * t4325;
    t7243
}
