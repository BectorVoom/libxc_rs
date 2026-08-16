//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1286/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1286(t1889: f64, t3766: f64, t3841: f64, t1419: f64, t5477: f64, t16082: f64, t5439: f64, t16078: f64, t16060: f64, t5425: f64, t11332: f64, t3781: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16457 = t3766 * t1889 * t3841;
    let t16461 = t3766 * t5477 * t1419;
    let t16464 = t5439 * t16082;
    let t16467 = t5439 * t16078;
    let t16470 = t5425 * t16060;
    let t16474 = t11332 * t1889 * t3781;
    (t16457, t16461, t16464, t16467, t16470, t16474)
}
