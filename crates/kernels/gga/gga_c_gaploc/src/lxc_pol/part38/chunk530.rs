//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 530/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk530<F: Float>(t1024: F, t2717: F, t2508: F, t2927: F, t954: F, t3464: F, t702: F, t3433: F, t779: F, t3431: F, t835: F, t723: F, t2580: F, t3448: F, t7137: F, t795: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10770 = t2717 * t1024;
    let t10772 = 0.76905262301422242837e-2 * t2508 * t10770;
    let t10773 = t954 * t2927;
    let t10775 = 0.76905262301422242837e-2 * t2508 * t10773;
    let t10776 = t3464 * t702;
    let t10779 = t779 * t3433;
    let t10782 = t835 * t3431;
    let t10783 = t10782 * t723;
    let t10784 = t2580 * t10783;
    let t10788 = 0.20508069947045931423e-1 * t7137 * t3448;
    let t10789 = t795 * t3431;
    (t10772, t10775, t10776, t10779, t10782, t10783, t10784, t10788, t10789)
}
