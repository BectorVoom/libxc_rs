//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2336/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2336(t24574: f64, t29557: f64, t29551: f64, t8003: f64, t94490: f64, t103218: f64, t103490: f64, t103494: f64, t103538: f64, t103577: f64, t103624: f64, t103659: f64, t103693: f64, t103733: f64, t103766: f64, t103801: f64, t103829: f64, t103864: f64, t103889: f64, t103918: f64, t103949: f64, t103978: f64, t104002: f64, t104482: f64, t1186: f64, t1238: f64, t1241: f64, t14980: f64, t1716: f64, t2122: f64, t24567: f64, t24638: f64, t27411: f64, t27751: f64, t29545: f64, t29670: f64, t4928: f64, t497: f64, t6146: f64, t7283: f64, t7303: f64, t8088: f64, t94710: f64) -> f64 {
    let t104502 = t24574 * t29557;
    let t104504 = t24574 * t29551;
    let t104506 = t94490 * t8003;
    let t104508 = -0.16449340668482264365e-1_f64 * t7283 * t1186 * t103490 - 0.54831135561607547883e-2_f64 * t103494 + 0.82246703342411321825e-2_f64 * t7283 * t6146 * t24638 - 0.16449340668482264365e-1_f64 * t7283 * t1716 * t2122 * t497 * t4928 - t1238 * t1241 * (t103538 + t103577 + t103624 + t103659 + t103693 + t103733 + t103766 + t103801 + t103829 + t103864 + t103889 + t103918 + t103949 + t103978 + t104002 + t104482) + 0.3289868133696452873e-1_f64 * t7283 * t27751 * t27411 - t94710 - 0.82246703342411321825e-2_f64 * t7283 * t1186 * t29670 - 2.0_f64 * t14980 * t8088 - 0.82246703342411321825e-2_f64 * t7283 * t24567 * t29545 - 0.80418998823691070228e-1_f64 * t103218 * t7303 + 0.54831135561607547883e-2_f64 * t104502 - 0.27415567780803773942e-2_f64 * t104504 + 0.48738787165873375897e-2_f64 * t104506;
    t104508
}
