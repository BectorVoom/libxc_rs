//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1022/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1022<F: Float>(t11384: F, t26836: F, t11499: F, t1700: F, t633: F, t1040: F, t3687: F, t8863: F, t3115: F, t436: F, t8780: F, t34550: F, t34553: F, t34555: F, t34557: F, t34560: F, t34563: F, t34565: F) -> (F,) {
    let t34567 = t11384 * t26836;
    let t34570 = t633 * t11499 * t1700;
    let t34573 = t8863 * t3687 * t1040;
    let t34576 = t3115 * t436 * t8780;
    let t34578 = 0.45289771048911752714e-7 * t34550 + 0.67530371184977617164e-6 * t34553 + 0.67530371184977617164e-6 * t34555 + 0.33765185592488808582e-6 * t34557 + 0.52838066223730378166e-7 * t34560 - 0.58366874983904959946e-8 * t34563 - 0.6629778687778673199e-7 * t34565 - 0.33148893438893365995e-7 * t34567 + 0.687148483626368822e-6 * t34570 - 0.33765185592488808582e-6 * t34573 - 0.45020247456651744776e-7 * t34576;
    (t34578,)
}
