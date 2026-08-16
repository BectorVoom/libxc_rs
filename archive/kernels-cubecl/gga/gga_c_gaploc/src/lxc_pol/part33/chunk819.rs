//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 819/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk819<F: Float>(t169: F, t172: F, t7861: F, t452: F, t493: F, t492: F, t197: F, t986: F, t161: F, t1367: F, t475: F) -> (F, F, F, F, F, F, F, F) {
    let t7863 = t7861 * t169 * t172;
    let t7864 = t452 * t7863;
    let t7879 = t493 * t7861;
    let t7880 = t492 * t7879;
    let t7887 = t197 * t986;
    let t7888 = t7887 * t161;
    let t7889 = t7888 * t1367;
    let t7892 = t986 * t475;
    (t7863, t7864, t7879, t7880, t7887, t7888, t7889, t7892)
}
