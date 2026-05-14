//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1014/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1014<F: Float>(t11347: F, t620: F, t1929: F, t3670: F, t11537: F, t3137: F, t505: F, t5059: F, t674: F, t12744: F, t5407: F, t9113: F, t1643: F, t22327: F, t3679: F, t1266: F, t205: F, t3683: F) -> (F, F, F, F, F, F) {
    let t34426 = t11347 * t620;
    let t34428 = t3670 * t1929;
    let t34433 = t11537 * t3137 * t505 * t674 * t5059;
    let t34436 = t9113 * t12744 * t5407;
    let t34439 = t1643 * t3679 * t22327;
    let t34442 = t1266 * t3683 * t205;
    (t34426, t34428, t34433, t34436, t34439, t34442)
}
