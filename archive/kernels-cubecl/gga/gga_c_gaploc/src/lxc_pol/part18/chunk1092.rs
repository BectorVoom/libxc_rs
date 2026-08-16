//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1092/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1092<F: Float>(t1402: F, t2033: F, t3280: F, t2628: F, t7403: F, t1980: F, t7634: F, t9824: F, t7419: F, t948: F, t9796: F, t5241: F, t935: F) -> (F, F, F, F, F, F) {
    let t28249 = F::cast_from(0.17875244975925213335e0_f64) * t2033 * t1402 * t3280;
    let t28259 = F::cast_from(0.11916829983950142223e0_f64) * t7403 * t2628;
    let t28279 = t1980 * t7634;
    let t28281 = F::cast_from(0.59584149919750711116e-1_f64) * t28279 * t9824;
    let t28283 = t9796 * t948 * t7419;
    let t28284 = F::cast_from(0.1533717038156829987e1_f64) * t28283;
    let t28286 = t5241 * t935;
    (t28249, t28259, t28279, t28281, t28284, t28286)
}
