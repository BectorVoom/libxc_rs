//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 807/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk807<F: Float>(t12623: F, t2549: F, t10053: F, t2558: F, t943: F, t12604: F, t1902: F, t883: F, t7064: F, t9756: F, t9624: F, t9647: F, t9648: F) -> (F, F, F, F, F, F) {
    let t40752 = t2549 * t12623;
    let t40758 = t943 * t10053 * t2558;
    let t40775 = t2549 * t12604;
    let t40820 = t883 * t1902;
    let t40822 = t7064 * t9756 * t40820;
    let t40825 = t9647 * t9648 * t9624;
    (t40752, t40758, t40775, t40820, t40822, t40825)
}
