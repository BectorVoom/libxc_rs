//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1219/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1219<F: Float>(t12744: F, t5407: F, t9113: F, t1643: F, t22327: F, t3679: F, t1266: F, t205: F, t3683: F, t144: F, t3095: F, t3094: F, t3954: F) -> (F, F, F, F, F) {
    let t34436 = t9113 * t12744 * t5407;
    let t34439 = t1643 * t3679 * t22327;
    let t34442 = t1266 * t3683 * t205;
    let t34447 = t3095 * t144;
    let t34449 = t3094 * t34447 * t3954;
    (t34436, t34439, t34442, t34447, t34449)
}
