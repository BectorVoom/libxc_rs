//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 833/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk833(t5425: f64, t5427: f64, t1319: f64, t1889: f64, t3761: f64, t1419: f64, t3766: f64, t1471: f64, t544: f64, t1444: f64, t1650: f64) -> (f64, f64, f64, f64, f64) {
    let t5428 = t5425 * t5427;
    let t5432 = t3761 * t1889 * t1319;
    let t5436 = t3766 * t1889 * t1419;
    let t5439 = t1471 * t544;
    let t5440 = t1444 * t1650;
    (t5428, t5432, t5436, t5439, t5440)
}
