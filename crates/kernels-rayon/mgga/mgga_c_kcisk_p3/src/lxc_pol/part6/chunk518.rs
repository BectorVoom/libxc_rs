//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 518/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk518(t2163: f64, t3973: f64, t1309: f64, t3951: f64, t9: f64, t403: f64, t3936: f64, t2169: f64, t25: f64, t1310: f64, t398: f64, t2168: f64, t3959: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6171 = t3973 * t2163;
    let t6172 = t1309 * t6171;
    let t6174 = t9 * t3951;
    let t6175 = t6174 * t403;
    let t6183 = t3936 * t403;
    let t6196 = t25 * t2169;
    let t6197 = t1309 * t6196;
    let t6204 = t1310 * t398;
    let t6205 = t3959 * t2168;
    (t6171, t6172, t6174, t6175, t6183, t6196, t6197, t6204, t6205)
}
