//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 493/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk493<F: Float>(t3869: F, t3903: F, t1216: F, t1349: F, t1402: F, t338: F, t3729: F, t3814: F, t3815: F, t3817: F, t3819: F, t3820: F, t3824: F, t3827: F, t3832: F, t3835: F, t417: F, t451: F) -> (F, F) {
    let t3904 = t3869 + t3903;
    let t3906 = t3814 + F::new(0.46853067927761790996e-2) * t3815 + F::new(0.93706135855523581992e-2) * t3817 + F::new(0.46853067927761790996e-2) * t3819 * t3820 + F::new(0.93706135855523581992e-2) * t1349 * t3824 - F::new(0.23426533963880895498e-2) * t1349 * t3827 + F::new(0.14055920378328537299e-1) * t417 * t3832 - F::new(0.46853067927761790996e-2) * t417 * t3835 - t3729 * t451 - F::new(2.0) * t1216 * t1402 - t338 * t3904;
    (t3904, t3906)
}
