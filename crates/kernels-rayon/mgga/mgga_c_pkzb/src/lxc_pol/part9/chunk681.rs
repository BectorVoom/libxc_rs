//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 681/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk681(t1167: f64, t931: f64, t824: f64, t2888: f64, t154: f64, t3026: f64, t907: f64, t178: f64, t2365: f64, t2364: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3175 = t931 * t1167;
    let t3176 = t3175 * t824;
    let t3177 = t2888 * t3176;
    let t3181 = t154 * t907 * t3026;
    let t3184 = t2365 * t178;
    let t3185 = t2364 * t3184;
    (t3175, t3176, t3177, t3181, t3184, t3185)
}
