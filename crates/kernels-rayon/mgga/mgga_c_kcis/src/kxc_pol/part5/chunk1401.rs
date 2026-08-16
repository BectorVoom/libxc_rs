//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1401/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1401(t1889: f64, t6183: f64, t4440: f64, t2104: f64, t5477: f64, t18128: f64, t1610: f64, t6912: f64, t21106: f64, t6159: f64, t6937: f64, t12617: f64) -> (f64, f64, f64, f64, f64) {
    let t23114 = t1889 * t6183;
    let t23115 = t4440 * t23114;
    let t23118 = t5477 * t2104;
    let t23119 = t18128 * t23118;
    let t23122 = t6912 * t1610;
    let t23123 = t4440 * t23122;
    let t23126 = t6159 * t21106;
    let t23129 = t6937 * t1610;
    let t23130 = t12617 * t23129;
    (t23115, t23119, t23123, t23126, t23130)
}
