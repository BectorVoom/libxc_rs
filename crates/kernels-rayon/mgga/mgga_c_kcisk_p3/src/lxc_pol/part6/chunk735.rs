//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 735/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk735(t14057: f64, t445: f64, t3845: f64, t429: f64, t431: f64, t1049: f64, t442: f64, t13964: f64, t12951: f64, t167: f64, t3532: f64, t967: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14059 = 0.72818958333333333333e-4_f64 * t445 * t14057;
    let t14062 = 0.27323333333333333333e-1_f64 * t429 * t3845 * t431;
    let t14082 = t1049 * t442;
    let t14083 = 0.62154466893555682512e-3_f64 * t14082;
    let t14084 = 0.71734315950379065738e-1_f64 * t13964;
    let t14085 = t167 * t12951;
    let t14090 = t967 * t3532;
    (t14059, t14062, t14083, t14084, t14085, t14090)
}
