//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1320/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1320<F: Float>(t1210: F, t29199: F, t1203: F, t21471: F, t1214: F, t1248: F, t1287: F, t21472: F, t2149: F, t2150: F, t26889: F, t26895: F, t26897: F, t26922: F, t26944: F, t26949: F, t26958: F, t26959: F, t26962: F, t26979: F, t26987: F, t26995: F, t29166: F, t473: F, t7637: F, t7639: F, t7659: F, t96954: F, t96981: F, t97011: F, t97078: F, t97082: F, t97095: F, t97299: F, t97304: F, t97308: F, t97313: F, t97314: F) -> F {
    let t97318 = t1210 * t29199;
    let t97319 = t21471 * t1203;
    let t97323 = F::cast_from(0.26020884564615598386e1_f64) * t26922 * t97011 * t29166 - F::cast_from(0.26020884564615598386e1_f64) * t97078 * t7639 + F::cast_from(0.52041769129231196772e1_f64) * t97082 * t26897 - F::cast_from(0.26020884564615598386e1_f64) * t26889 * t26962 * t1248 * t1287 - F::cast_from(0.78062653693846795158e1_f64) * t26949 * t7637 * t26958 * t1214 + F::cast_from(0.26020884564615598386e1_f64) * t26979 * t26959 - F::cast_from(0.13010442282307799193e1_f64) * t7659 * t97095 * t1248 * t1287 + F::cast_from(0.26020884564615598386e1_f64) * t26922 * t26987 * t1248 * t1287 + F::cast_from(0.52041769129231196772e1_f64) * t26922 * t26944 * t1248 * t1287 + F::cast_from(0.26020884564615598386e1_f64) * t26895 * t26958 * t1248 * t1287 - F::cast_from(0.4336814094102599731e0_f64) * t2149 * t2150 * t473 * t97299 + F::cast_from(0.10408353825846239354e2_f64) * t97304 * t26995 * t96954 - F::cast_from(0.26020884564615598386e1_f64) * t97308 * t96981 * t21472 + F::cast_from(0.52041769129231196772e1_f64) * t97313 * t96981 * t97314 + F::cast_from(0.26020884564615598386e1_f64) * t97318 * t96981 * t97319;
    t97323
}
