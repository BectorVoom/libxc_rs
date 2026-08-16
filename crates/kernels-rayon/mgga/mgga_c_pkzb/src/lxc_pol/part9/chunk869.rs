//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 869/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk869(t2320: f64, t6122: f64, t2234: f64, t853: f64, t2197: f64, t2242: f64, t851: f64, t2240: f64, t2312: f64, t891: f64, t889: f64, t2273: f64, t872: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6324 = t6122 * t2320;
    let t6327 = t853 * t2234;
    let t6329 = 6.0_f64 * t2197 * t6327;
    let t6331 = t2234 * t2242 * t851;
    let t6333 = 0.48245938496077605201e2_f64 * t2240 * t6331;
    let t6334 = t891 * t2312;
    let t6337 = t2312 * t2320;
    let t6338 = t6337 * t889;
    let t6341 = t872 * t2273;
    (t6324, t6327, t6329, t6331, t6333, t6334, t6337, t6338, t6341)
}
