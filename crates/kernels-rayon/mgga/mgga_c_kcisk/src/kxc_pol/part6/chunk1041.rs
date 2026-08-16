//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1041/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1041(t1433: f64, t30941: f64, t457: f64, t2192: f64, t7897: f64, t5953: f64, t1422: f64, t1423: f64, t30158: f64, t2191: f64, t26590: f64, t5926: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31100 = t1433 * t30941;
    let t31101 = t457 * t31100;
    let t31106 = t2192 * t7897;
    let t31107 = t5953 * t31106;
    let t31111 = t1422 * t1423 * t30158;
    let t31114 = t26590 * t2191;
    let t31115 = t5926 * t31114;
    (t31100, t31101, t31106, t31107, t31111, t31114, t31115)
}
