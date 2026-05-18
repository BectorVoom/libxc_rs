//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 965/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk965<F: Float>(t312: F, t4242: F, t4245: F, t4249: F, t4296: F, t4301: F, t4304: F, t4307: F, t4318: F, t4322: F, t4324: F, t4325: F, t5893: F, t5896: F, t5901: F, t61: F, t7124: F, t7149: F, t7170: F, t7236: F) -> F {
    let t7243 = -t5893 - F::new(0.02394846802050922) * t5896 - F::new(3.64371538634302e-05) * t5901 + (t7124 + t7149) * t312 + (t7170 + t7236) * t61 + t4242 - F::new(1.82185769317151e-05) * t4245 - t4249 - t4296 - t4301 + F::new(0.039914113367515366) * t4304 + t4307 - F::new(0.01197423401025461) * t4318 + t4322 - t4324 - F::new(0.05321881782335382) * t4325;
    t7243
}
