//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 680/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk680(t2320: f64, t889: f64, t3160: f64, t898: f64, t1220: f64, t904: f64, t1167: f64, t154: f64, t2347: f64, t385: f64, t23: f64, t2886: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3161 = t2320 * t889;
    let t3162 = t3160 * t3161;
    let t3164 = 0.17315859105681463759e2_f64 * t898 * t3162;
    let t3165 = t1220 * t904;
    let t3171 = t154 * t2347 * t1167;
    let t3172 = t385 * t3171;
    let t3174 = t23 * t2886;
    (t3161, t3162, t3164, t3165, t3172, t3174)
}
