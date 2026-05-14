//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 855/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk855<F: Float>(t7640: F, t898: F, t1476: F, t7242: F, t25162: F, t33849: F, t2253: F, t6307: F, t33824: F, t33828: F, t33288: F, t33831: F, t7638: F, t33836: F, t2: F, t33953: F) -> (F, F, F, F, F, F, F, F, F) {
    let t143041 = t898 * t7640;
    let t143042 = t7242 * t1476;
    let t143058 = t25162 * t33849;
    let t143100 = t6307 * t2253;
    let t143101 = t143100 * t33824;
    let t143112 = t898 * t33828;
    let t143120 = t7638 * t33288 * t33831;
    let t143123 = t7638 * t33288 * t33836;
    let t143144 = t2 * t33953;
    (t143041, t143042, t143058, t143100, t143101, t143112, t143120, t143123, t143144)
}
