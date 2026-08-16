//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 918/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk918(t2035: f64, t2363: f64, t2094: f64, t40611: f64, t12461: f64, t7216: f64, t193: f64, t7125: f64, t2053: f64, t40889: f64, t10109: f64, t7106: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91857 = t2035 * t2363;
    let t92169 = t2094 * t40611;
    let t92200 = t7216 * t12461;
    let t92271 = t193 * t7125;
    let t92394 = t40889 * t2053;
    let t92981 = t10109 * t7106;
    (t91857, t92169, t92200, t92271, t92394, t92981)
}
