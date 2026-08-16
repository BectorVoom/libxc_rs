//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2254/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2254(t1985: f64, t6907: f64, t90739: f64, t22685: f64, t22686: f64, t26193: f64, t16018: f64, t6888: f64, t6889: f64, t6890: f64, t22674: f64, t22892: f64, t26189: f64) -> (f64, f64, f64, f64) {
    let t91469 = t1985 * t90739 * t6907;
    let t91478 = t22685 * t26193 * t22686;
    let t91482 = t6888 * t6889 * t6890 * t16018;
    let t91486 = t22892 * t22674 * t26189;
    (t91469, t91478, t91482, t91486)
}
