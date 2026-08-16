//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 894/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk894<F: Float>(t532: F, t8807: F, t31085: F, t2039: F, t7156: F, t111: F, t8710: F) -> (F, F, F, F) {
    let t32212 = t532 * t8807;
    let t32213 = t32212 * t31085;
    let t32220 = t7156 * t2039;
    let t32235 = t8710 * t111;
    (t32212, t32213, t32220, t32235)
}
