//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 749/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk749(t1933: f64, t3970: f64, t1368: f64, t3978: f64, t498: f64, t5427: f64, t1380: f64, t1889: f64, t3984: f64, t1370: f64, t5441: f64, t1369: f64, t736: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5698 = t3970 * t1933;
    let t5699 = t1368 * t5698;
    let t5701 = t3978 * t498;
    let t5702 = t5701 * t5427;
    let t5705 = t1889 * t1380;
    let t5706 = t3984 * t5705;
    let t5709 = t1370 * t498;
    let t5710 = t5709 * t5441;
    let t5713 = t736 * t1369;
    (t5698, t5699, t5701, t5702, t5705, t5706, t5709, t5710, t5713)
}
