//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 958/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk958<F: Float>(t41231: F, t41244: F, t39118: F, t959: F, t39123: F, t43761: F, t43762: F, t43766: F, t43768: F, t43771: F, t43774: F, t43776: F, t43777: F, t43781: F, t13847: F, t2684: F, t7354: F) -> (F, F) {
    let t47377 = 0.63904876589867916128e-1 * t41231;
    let t47378 = 0.63904876589867916128e-1 * t41244;
    let t47379 = t39118 * t959;
    let t47381 = t39123 * t959;
    let t47383 = t43761 + 0.11916829983950142223e0 * t43762 + t43766 + 0.42900587942220512003e1 * t43768 - 0.21450293971110256001e1 * t43771 + t43774 + t47377 - t43776 + t43777 - t47378 + 0.14896037479937677779e-1 * t47379 + 0.14896037479937677779e-1 * t47381 + t43781;
    let t47389 = t2684 * t7354 * t13847;
    (t47383, t47389)
}
