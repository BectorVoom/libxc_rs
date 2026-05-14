//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1394/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1394<F: Float>(t104173: F, t104214: F, t104239: F, t104286: F, t104330: F, t104375: F, t104405: F, t104442: F, t104475: F, t104515: F, t104550: F, t104580: F, t104621: F, t106173: F, t107699: F, t107734: F, t12092: F, t12244: F, t12257: F, t13256: F, t1395: F, t1577: F, t1580: F, t184: F, t21: F, t2301: F, t2306: F, t24153: F, t24157: F, t27436: F, t27440: F, t363: F, t3674: F, t3678: F, t5: F, t5985: F, t6732: F, t920: F) -> (F,) {
    let t107745 = t5985 * t12257 / 2.0 + t5985 * t12092 + t5 * t6732 * t1580 / 4.0 + t5 * t27436 * t363 / 2.0 + t5 * t24153 * t920 / 4.0 + t24157 * t3674 / 2.0 + t24157 * t3678 + t27440 * t2301 / 4.0 + t5985 * t12244 / 4.0 + t5 * t1395 * t1577 / 2.0 + t27440 * t2306 / 4.0 + t5 * (t104173 + t104214 + t104239 + t104286 + t104330 + t104375 + t104405 + t104442 + t104475 + t104515 + t104550 + t104580 + t104621 + t106173 + t107699 + t107734) * t184 * t21 / 4.0 + t5985 * t13256 / 2.0;
    (t107745,)
}
