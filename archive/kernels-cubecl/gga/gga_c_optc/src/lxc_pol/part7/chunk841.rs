//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 841/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk841<F: Float>(t2648: F, t7178: F, t894: F, t2813: F, t8040: F, t297: F, t312: F, t7835: F, t287: F, t914: F, t2712: F, t2715: F) -> (F, F, F, F, F, F, F, F) {
    let t8048 = t2648 * t7178;
    let t8049 = t894 * t8048;
    let t8052 = t2813 * t8040;
    let t8057 = t312 * t7835 * t297;
    let t8058 = t894 * t8057;
    let t8062 = t287 * t7835 * t297;
    let t8063 = t914 * t8062;
    let t8066 = t2712 * t2715;
    (t8048, t8049, t8052, t8057, t8058, t8062, t8063, t8066)
}
