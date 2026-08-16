//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 976/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk976(t4948: f64, t831: f64, t486: f64, t6616: f64, t132: f64, t1547: f64, t2583: f64, t2470: f64, t3223: f64, t1447: f64, t6120: f64, t2477: f64, t3213: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16294 = t831 * t4948;
    let t16298 = t486 * t6616;
    let t16305 = t132 * t1547 * t2583;
    let t16307 = t3223 * t2470;
    let t16309 = t1447 * t6120;
    let t16314 = t3213 * t2477;
    (t16294, t16298, t16305, t16307, t16309, t16314)
}
