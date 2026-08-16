//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1029/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1029(t1472: f64, t6236: f64, t3416: f64, t6239: f64, t13080: f64, t1318: f64, t6478: f64, t4753: f64, t10467: f64, t2396: f64, t519: f64, t4763: f64, t5282: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17692 = t1472 * t6236;
    let t17694 = t3416 * t6239;
    let t17697 = t1318 * t13080 * t6478;
    let t17699 = t4753 * t6239;
    let t17709 = t519 * t10467 * t2396;
    let t17715 = t4763 * t5282;
    (t17692, t17694, t17697, t17699, t17709, t17715)
}
