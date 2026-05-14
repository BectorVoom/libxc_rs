//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 840/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk840<F: Float>(t326: F, t43508: F, t825: F, t2684: F, t7585: F, t43490: F, t13145: F, t2033: F, t549: F, t33360: F, t787: F, t9824: F, t2028: F, t42921: F, t43462: F, t43465: F, t43468: F, t43471: F, t43477: F, t43479: F, t43481: F, t43484: F, t43489: F, t43492: F, t43497: F, t43500: F, t43502: F, t43505: F) -> (F,) {
    let t43511 = 0.92023022289409799224e1 * t825 * t326 * t43508;
    let t43514 = 0.43710935587469654631e2 * t2684 * t7585 * t43508;
    let t43516 = t2684 * t7585 * t43490;
    let t43519 = t2033 * t549 * t13145;
    let t43522 = t787 * t33360 * t9824;
    let t43523 = 0.29792074959875355558e-1 * t43522;
    let t43524 = 0.29792074959875355558e-1 * t43462 + t43465 + t43468 + t43471 + 0.39722766613167140743e-1 * t2033 * t549 * t42921 - t43477 - t43479 - 0.21450293971110256002e1 * t43481 - 0.21450293971110256002e1 * t43484 - t43489 - 0.18404604457881959845e2 * t43492 - t43497 + t43500 + 0.29792074959875355558e-1 * t43502 - 0.39722766613167140743e-1 * t43505 * t2028 - t43511 + t43514 + 0.87421871174939309263e2 * t43516 + 0.59584149919750711116e-1 * t43519 + t43523;
    (t43524,)
}
