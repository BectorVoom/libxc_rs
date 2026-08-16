//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1320/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1320(t1167: f64, t154: f64, t19023: f64, t385: f64, t3214: f64, t6467: f64, t1229: f64, t17955: f64, t918: f64, t1238: f64, t6428: f64, t6476: f64, t8319: f64) -> (f64, f64, f64, f64, f64) {
    let t23338 = t385 * t154 * t19023 * t1167;
    let t23340 = t3214 * t6467;
    let t23341 = 0.7622047665434619906e-3_f64 * t23340;
    let t23345 = t918 * t17955 * t1229;
    let t23355 = t1238 * t6428;
    let t23362 = t8319 * t6476;
    (t23338, t23341, t23345, t23355, t23362)
}
