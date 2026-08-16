//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2336/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2336<F: Float>(t24574: F, t29557: F, t29551: F, t8003: F, t94490: F, t103218: F, t103490: F, t103494: F, t103538: F, t103577: F, t103624: F, t103659: F, t103693: F, t103733: F, t103766: F, t103801: F, t103829: F, t103864: F, t103889: F, t103918: F, t103949: F, t103978: F, t104002: F, t104482: F, t1186: F, t1238: F, t1241: F, t14980: F, t1716: F, t2122: F, t24567: F, t24638: F, t27411: F, t27751: F, t29545: F, t29670: F, t4928: F, t497: F, t6146: F, t7283: F, t7303: F, t8088: F, t94710: F) -> F {
    let t104502 = t24574 * t29557;
    let t104504 = t24574 * t29551;
    let t104506 = t94490 * t8003;
    let t104508 = -F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1186 * t103490 - F::cast_from(0.54831135561607547883e-2_f64) * t103494 + F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t6146 * t24638 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1716 * t2122 * t497 * t4928 - t1238 * t1241 * (t103538 + t103577 + t103624 + t103659 + t103693 + t103733 + t103766 + t103801 + t103829 + t103864 + t103889 + t103918 + t103949 + t103978 + t104002 + t104482) + F::cast_from(0.3289868133696452873e-1_f64) * t7283 * t27751 * t27411 - t94710 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t1186 * t29670 - F::cast_from(2.0_f64) * t14980 * t8088 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t24567 * t29545 - F::cast_from(0.80418998823691070228e-1_f64) * t103218 * t7303 + F::cast_from(0.54831135561607547883e-2_f64) * t104502 - F::cast_from(0.27415567780803773942e-2_f64) * t104504 + F::cast_from(0.48738787165873375897e-2_f64) * t104506;
    t104508
}
