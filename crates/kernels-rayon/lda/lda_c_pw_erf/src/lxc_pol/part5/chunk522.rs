//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 522/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk522(t2610: f64, t436: f64, t127: f64, t1655: f64, t1663: f64, t1674: f64, t1689: f64, t1695: f64, t1838: f64, t1850: f64, t2598: f64, t2601: f64, t2613: f64, t2616: f64, t2620: f64, t2624: f64, t426: f64) -> (f64, f64) {
    let t2627 = t436 * t2610;
    let t2630 = -t1655 + t2598 + t1663 + t2601 - t2613 + t1674 + t1838 / 3.0_f64 + 3.0_f64 / 2.0_f64 * t426 * t2616 - t426 * t2620 / 2.0_f64 + t1689 + 1.46904_f64 * t1850 + t1695 + 5.87616_f64 * t127 * t2624 - 1.46904_f64 * t127 * t2627;
    (t2627, t2630)
}
