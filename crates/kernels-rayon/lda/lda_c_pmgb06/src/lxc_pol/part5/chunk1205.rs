//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1205/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1205(t248: f64, t686: f64, t7402: f64, t11132: f64, t11133: f64, t11136: f64, t11140: f64, t11141: f64, t15015: f64, t8755: f64, t8759: f64, t8760: f64, t8762: f64, t8769: f64, t8774: f64, t8779: f64, t8787: f64, t8794: f64, t8798: f64) -> f64 {
    let t21801 = t248 * t7402 * t686;
    let t21803 = -t8755 - t8759 + 3.5089341735807875_f64 * t8760 - 51.94757731704439_f64 * t8762 + t8769 - t8774 + t8779 + t11132 - t11133 - 0.0005696894717424259_f64 * t8787 - t8794 + t21801 - t11136 + t11140 + 3.5089341735807875_f64 * t15015 - t8798 + t11141;
    t21803
}
