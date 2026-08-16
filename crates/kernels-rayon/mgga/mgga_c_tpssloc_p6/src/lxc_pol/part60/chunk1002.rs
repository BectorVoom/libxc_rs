//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1002/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1002(t127908: f64, t127916: f64, t101355: f64, t101509: f64, t112863: f64, t114865: f64, t114892: f64, t121629: f64, t126372: f64, t126385: f64, t126398: f64, t126399: f64, t126404: f64, t127889: f64, t127896: f64, t1912: f64, t218: f64, t25348: f64, t259: f64, t28317: f64, t28432: f64, t7087: f64, t7842: f64) -> (f64, f64) {
    let t127917 = t127908 + t127916;
    let t127926 = t126372 + t112863 - t101355 * t1912 + 0.3289868133696452873e-1_f64 * t127889 - 2.0_f64 * t25348 * t7842 - t126385 + 0.82246703342411321825e-2_f64 * t127896 - t114865 + t114892 + t218 * t127917 * t259 - 0.38381794893125283518e-1_f64 * t121629 - t126398 + 2.0_f64 * t7087 * t28317 + t126399 + t126404 - t7087 * t28432 - 2.0_f64 * t101509 * t1912;
    (t127917, t127926)
}
