//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1079/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1079(t12633: f64, t2188: f64, t3709: f64, t2171: f64, t3784: f64, t3788: f64, t4738: f64, t5068: f64, t518: f64, t2168: f64, t10508: f64, t826: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12634 = 8.0_f64 / 9.0_f64 * t12633;
    let t12636 = 4.0_f64 / 5.0_f64 * t3709 * t2188;
    let t12637 = t2171 * t3784;
    let t12638 = 8.0_f64 / 135.0_f64 * t12637;
    let t12639 = t4738 * t3788;
    let t12640 = 16.0_f64 / 15.0_f64 * t12639;
    let t12641 = t5068 * t518;
    let t12643 = 8.0_f64 / 5.0_f64 * t12641 * t2168;
    let t12645 = 8.0_f64 / 15.0_f64 * t10508 * t826;
    (t12634, t12636, t12638, t12640, t12641, t12643, t12645)
}
