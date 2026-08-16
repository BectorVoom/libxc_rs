//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 534/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk534(t1158: f64, t1165: f64, t1169: f64, t1181: f64, t1189: f64, t1195: f64, t1740: f64, t1885: f64, t1898: f64, t2311: f64, t2313: f64, t2680: f64) -> f64 {
    let t2685 = -0.0005811348303577384_f64 * t1898 - 0.02394846802050922_f64 * t2311 + 0.039914113367515366_f64 * t2313 - 0.10809180959278285_f64 * t1885 + t1158 - t1165 + t1169 - t1181 - t1189 + t1195 - t1740;
    let t2686 = t2680 + t2685;
    t2686
}
