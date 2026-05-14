//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 815/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk815<F: Float>(t41448: F, t41477: F, t2558: F, t39002: F, t9647: F, t12311: F, t2554: F, t7064: F, t1843: F, t47178: F, t39040: F, t5539: F, t13934: F, t731: F, t13937: F, t2549: F) -> (F, F, F, F, F, F, F, F) {
    let t47555 = 0.31952438294933958064e0 * t41448;
    let t47558 = 0.12780975317973583226e0 * t41477;
    let t47594 = t9647 * t39002 * t2558;
    let t47597 = t7064 * t12311 * t2554;
    let t47607 = t9647 * t1843 * t47178;
    let t47610 = t9647 * t5539 * t39040;
    let t47652 = t731 * t13934;
    let t47687 = t2549 * t13937;
    (t47555, t47558, t47594, t47597, t47607, t47610, t47652, t47687)
}
