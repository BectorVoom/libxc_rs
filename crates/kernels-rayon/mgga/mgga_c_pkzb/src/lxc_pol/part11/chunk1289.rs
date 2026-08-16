//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1289/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1289(t2240: f64, t3069: f64, t9859: f64, t11243: f64, t6199: f64, t851: f64, t10000: f64, t10003: f64, t11180: f64, t11214: f64, t11217: f64, t18513: f64, t18866: f64, t18878: f64, t2296: f64, t2318: f64, t3135: f64, t31357: f64, t31369: f64, t31372: f64, t31375: f64, t31377: f64, t31380: f64, t6282: f64, t8120: f64, t8211: f64, t889: f64, t9985: f64, t9992: f64) -> (f64, f64, f64) {
    let t31383 = 0.48245938496077605201e2_f64 * t2240 * t9859 * t3069;
    let t31390 = 0.57895126195293126241e3_f64 * t6199 * t11243 * t851;
    let t31391 = 0.51947577317044391277e2_f64 * t2318 * t9985 * t3135 - 0.12304822629859687989e5_f64 * t18866 * t11217 * t889 - 0.11696447245269292414e1_f64 * t2296 * t11214 * t889 + 0.17315859105681463759e2_f64 * t2318 * t31357 * t889 + 0.30762056574649219974e4_f64 * t6282 * t9992 * t3135 + 0.91082604192152556044e5_f64 * t18878 * t11180 * t18513 * t889 + t31369 + t31372 + t31375 - t31377 - t31380 - t31383 + 18.0_f64 * t8120 * t10000 - 12.0_f64 * t8211 * t10003 - t31390;
    (t31383, t31390, t31391)
}
