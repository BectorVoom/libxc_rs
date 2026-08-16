//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 714/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk714(t1031: f64, t3137: f64, t3242: f64, t980: f64, t177: f64, t1001: f64, t3139: f64, t214: f64, t1035: f64, t9352: f64, t3127: f64, t981: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12693 = t1031 * t3137;
    let t12694 = t12693 * t3242;
    let t12696 = t980 * t980;
    let t12697 = 1.0_f64 / t12696;
    let t12698 = t177 * t12697;
    let t12699 = t3139 * t1001;
    let t12700 = t214 * t12699;
    let t12701 = t12698 * t12700;
    let t12703 = t1035 * t9352;
    let t12705 = t3127 * t981;
    (t12694, t12697, t12698, t12699, t12701, t12703, t12705)
}
