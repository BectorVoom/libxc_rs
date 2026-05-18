//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 795/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk795<F: Float>(t2568: F, t9454: F, t291: F, t7549: F, t8820: F, t7547: F, t871: F, t903: F, t2526: F, t2505: F, t904: F, t1: F, t282: F, t3: F) -> (F, F, F, F, F) {
    let t9457 = t2568 * t9454;
    let t9460 = t8820 * t291 * t7549;
    let t9461 = t7547 * t9460;
    let t9463 = t871 * t903;
    let t9464 = t9463 * t2526;
    let t9468 = t904 * t2505;
    let t9471 = t282 * t1 * t3;
    (t9457, t9461, t9464, t9468, t9471)
}
