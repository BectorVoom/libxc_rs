//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1320/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1320(t1210: f64, t29199: f64, t1203: f64, t21471: f64, t1214: f64, t1248: f64, t1287: f64, t21472: f64, t2149: f64, t2150: f64, t26889: f64, t26895: f64, t26897: f64, t26922: f64, t26944: f64, t26949: f64, t26958: f64, t26959: f64, t26962: f64, t26979: f64, t26987: f64, t26995: f64, t29166: f64, t473: f64, t7637: f64, t7639: f64, t7659: f64, t96954: f64, t96981: f64, t97011: f64, t97078: f64, t97082: f64, t97095: f64, t97299: f64, t97304: f64, t97308: f64, t97313: f64, t97314: f64) -> f64 {
    let t97318 = t1210 * t29199;
    let t97319 = t21471 * t1203;
    let t97323 = 0.26020884564615598386e1_f64 * t26922 * t97011 * t29166 - 0.26020884564615598386e1_f64 * t97078 * t7639 + 0.52041769129231196772e1_f64 * t97082 * t26897 - 0.26020884564615598386e1_f64 * t26889 * t26962 * t1248 * t1287 - 0.78062653693846795158e1_f64 * t26949 * t7637 * t26958 * t1214 + 0.26020884564615598386e1_f64 * t26979 * t26959 - 0.13010442282307799193e1_f64 * t7659 * t97095 * t1248 * t1287 + 0.26020884564615598386e1_f64 * t26922 * t26987 * t1248 * t1287 + 0.52041769129231196772e1_f64 * t26922 * t26944 * t1248 * t1287 + 0.26020884564615598386e1_f64 * t26895 * t26958 * t1248 * t1287 - 0.4336814094102599731e0_f64 * t2149 * t2150 * t473 * t97299 + 0.10408353825846239354e2_f64 * t97304 * t26995 * t96954 - 0.26020884564615598386e1_f64 * t97308 * t96981 * t21472 + 0.52041769129231196772e1_f64 * t97313 * t96981 * t97314 + 0.26020884564615598386e1_f64 * t97318 * t96981 * t97319;
    t97323
}
