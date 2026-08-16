//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 942/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk942(t12368: f64, t2057: f64, t955: f64, t2054: f64, t12535: f64, t495: f64, t5065: f64, t132: f64, t1547: f64, t2042: f64, t1963: f64, t3213: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13595 = 0.03199259259259259_f64 * t12368;
    let t13619 = t955 * t2057;
    let t13621 = t955 * t2054;
    let t13672 = t5065 * t12535 * t495;
    let t13706 = t132 * t1547 * t2042;
    let t13707 = t13706 / 45.0_f64;
    let t13708 = t3213 * t1963;
    (t13595, t13619, t13621, t13672, t13707, t13708)
}
