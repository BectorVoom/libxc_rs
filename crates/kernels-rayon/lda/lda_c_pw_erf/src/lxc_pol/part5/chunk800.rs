//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 800/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk800(t5690: f64, t2748: f64, t2752: f64, t2755: f64, t2759: f64, t2761: f64, t2944: f64, t2950: f64, t2989: f64, t7330: f64, t7332: f64, t7333: f64, t7336: f64) -> (f64, f64) {
    let t7350 = 24.0_f64 * t5690;
    let t7351 = t7330 + t7332 + t7333 - t2748 + t2752 - t2755 + t2759 - t2761 - t7336 - t2944 + t2950 - t7350 - t2989;
    (t7350, t7351)
}
