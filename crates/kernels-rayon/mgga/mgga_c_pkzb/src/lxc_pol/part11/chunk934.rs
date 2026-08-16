//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 934/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk934(t3806: f64, t6230: f64, t6233: f64, t889: f64, t898: f64, t3147: f64, t3162: f64, t2295: f64, t3819: f64, t891: f64, t3840: f64, t2317: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10168 = t6230 * t3806;
    let t10169 = t6233 * t889;
    let t10170 = t10168 * t10169;
    let t10172 = 0.10254018858216406658e4_f64 * t898 * t10170;
    let t10174 = 0.34631718211362927517e2_f64 * t3147 * t3162;
    let t10175 = t2295 * t3819;
    let t10176 = t10175 * t891;
    let t10178 = 0.11696447245269292414e1_f64 * t898 * t10176;
    let t10179 = t3840 * t891;
    let t10181 = 0.35089341735807877242e1_f64 * t898 * t10179;
    let t10182 = t2317 * t3819;
    (t10168, t10169, t10170, t10172, t10174, t10176, t10178, t10179, t10181, t10182)
}
