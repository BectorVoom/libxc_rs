//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 819/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk819<F: Float>(t2684: F, t2685: F, t43710: F, t10628: F, t549: F, t6111: F, t24505: F, t9438: F, t3295: F, t8802: F, t9800: F, t13052: F, t1966: F) -> (F, F, F, F, F) {
    let t43712 = t2684 * t2685 * t43710;
    let t43715 = t6111 * t549 * t10628;
    let t43718 = t2684 * t9438 * t24505;
    let t43756 = t9800 * t8802 * t3295;
    let t43758 = t1966 * t13052;
    (t43712, t43715, t43718, t43756, t43758)
}
