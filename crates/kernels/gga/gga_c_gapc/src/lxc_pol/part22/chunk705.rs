//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 705/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk705<F: Float>(t3114: F, t8774: F, t1037: F, t2999: F, t520: F, t1689: F, t3006: F, t3115: F, t1: F, t116: F, t5054: F) -> (F, F, F, F, F, F, F) {
    let t8775 = t8774 * t3114;
    let t8776 = t1037 * t2999;
    let t8777 = t520 * t8776;
    let t8778 = t8775 * t8777;
    let t8780 = t1689 * t3006;
    let t8781 = t520 * t8780;
    let t8782 = t3115 * t8781;
    let t8784 = t116 * t1;
    let t8785 = 1.0 / t5054;
    (t8775, t8776, t8778, t8780, t8782, t8784, t8785)
}
