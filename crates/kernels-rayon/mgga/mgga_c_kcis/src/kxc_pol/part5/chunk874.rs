//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 874/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk874(t1889: f64, t1961: f64, t3766: f64, t1471: f64, t3771: f64, t6281: f64, t1472: f64, t6284: f64, t3780: f64, t6957: f64, t542: f64, t5463: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7221 = t1889 * t1961;
    let t7222 = t3766 * t7221;
    let t7226 = t1471 * t3771 * t6281;
    let t7230 = t1471 * t1472 * t6284;
    let t7233 = t3780 * t6957;
    let t7234 = t542 * t7233;
    let t7237 = t5463 * t1961;
    (t7222, t7226, t7230, t7233, t7234, t7237)
}
