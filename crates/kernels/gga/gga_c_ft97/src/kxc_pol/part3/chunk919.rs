//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 919/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk919<F: Float>(t17799: F, t18176: F, t661: F, t1882: F, t5161: F, t5157: F, t2469: F, t5073: F, t729: F, t1168: F, t3821: F, t762: F) -> (F, F, F, F, F) {
    let t18177 = t17799 + t18176;
    let t18178 = t661 * t18177;
    let t18188 = t1882 * t5161;
    let t18190 = t1882 * t5157;
    let t18193 = t729 * t2469 * t5073;
    let t18196 = t3821 * t1168;
    let t18198 = t729 * t762 * t18196;
    (t18178, t18188, t18190, t18193, t18198)
}
