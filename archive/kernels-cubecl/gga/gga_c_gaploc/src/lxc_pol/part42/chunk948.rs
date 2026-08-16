//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 948/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk948<F: Float>(t11981: F, t2464: F, t2465: F, t2487: F, t13782: F, t7014: F, t13791: F, t1429: F, t549: F, t13779: F, t1407: F, t38674: F, t544: F) -> (F, F, F, F, F) {
    let t47883 = t2487 * t2464 * t2465 * t11981;
    let t47885 = t7014 * t13782;
    let t47892 = t1429 * t549 * t13791;
    let t47949 = t1407 * t13779;
    let t47964 = t544 * t38674;
    (t47883, t47885, t47892, t47949, t47964)
}
