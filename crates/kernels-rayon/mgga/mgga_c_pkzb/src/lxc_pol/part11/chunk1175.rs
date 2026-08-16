//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1175/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1175(t28889: f64, t28910: f64, t98: f64, t126: f64, t83: f64, t10534: f64, t545: f64, t19620: f64, t19625: f64, t19627: f64, t16476: f64, t16193: f64, t16230: f64, t16273: f64, t16275: f64, t16280: f64, t16283: f64, t16287: f64, t16290: f64, t16481: f64, t16486: f64, t16489: f64, t19624: f64, t19688: f64, t19690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28912 = (t28889 + t28910) * t98;
    let t28914 = t83 * t28912 * t126;
    let t28916 = t83 * t10534 * t545;
    let t28917 = 0.17090684152272775384e-2_f64 * t19620;
    let t28918 = 0.48796115851357829289e-1_f64 * t19625;
    let t28919 = 0.14447919941302971323e1_f64 * t19627;
    let t28920 = 0.35089341735807877242e1_f64 * t16476;
    let t28921 = -t16193 + t28914 + t28916 - t16230 - t16273 + t16275 - t28917 + t19624 + t28918 + t28919 - t16280 + t16283 + t16287 - t16290 + t19688 + t19690 + t28920 + t16481 - t16486 - t16489;
    (t28912, t28914, t28916, t28917, t28918, t28919, t28920, t28921)
}
