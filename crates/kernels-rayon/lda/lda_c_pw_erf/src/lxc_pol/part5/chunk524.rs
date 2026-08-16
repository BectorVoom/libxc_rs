//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 524/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk524(t133: f64, t1655: f64, t1663: f64, t1717: f64, t1868: f64, t2598: f64, t2601: f64, t2613: f64, t2616: f64, t2620: f64) -> f64 {
    let t2642 = -t1655 + t2598 + t1663 + t2601 - t2613 + t1717 + 1.1495033333333333_f64 * t1868 + 5.172765_f64 * t133 * t2616 - 1.724255_f64 * t133 * t2620;
    t2642
}
