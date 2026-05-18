//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 819/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk819<F: Float>(t6301: F, t2466: F, t833: F, t3668: F, t186: F, t211: F, t6304: F, t6307: F, t2471: F, t806: F, t3676: F, t185: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7512 = F::new(4.0) / F::new(15.0) * t6301;
    let t7513 = t2466 * t833;
    let t7514 = t3668 * t7513;
    let t7515 = t186 * t7514;
    let t7517 = F::new(4.0) / F::new(5.0) * t211 * t7515;
    let t7518 = F::new(8.0) / F::new(15.0) * t6304;
    let t7519 = F::new(8.0) / F::new(15.0) * t6307;
    let t7520 = t2471 * t806;
    let t7521 = t3676 * t7520;
    let t7522 = t186 * t7521;
    let t7524 = F::new(4.0) / F::new(5.0) * t185 * t7522;
    (t7512, t7513, t7514, t7515, t7517, t7518, t7519, t7520, t7521, t7522, t7524)
}
