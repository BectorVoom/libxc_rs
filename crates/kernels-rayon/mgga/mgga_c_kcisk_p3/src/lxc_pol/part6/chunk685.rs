//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 685/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk685(t11530: f64, t445: f64, t1849: f64, t213: f64, t4597: f64, t967: f64, t10487: f64, t167: f64, t11458: f64, t1049: f64, t695: f64, t642: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11532 = 0.72818958333333333333e-4_f64 * t445 * t11530;
    let t11612 = t213 * t1849;
    let t11625 = t967 * t4597;
    let t11630 = t167 * t10487;
    let t11633 = 0.71734315950379065738e-1_f64 * t11458;
    let t11634 = t1049 * t695;
    let t11635 = 0.62154466893555682512e-3_f64 * t11634;
    let t11682 = t642 * t1849;
    (t11532, t11612, t11625, t11630, t11633, t11635, t11682)
}
