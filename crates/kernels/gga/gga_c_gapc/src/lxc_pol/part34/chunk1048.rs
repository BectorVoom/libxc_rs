//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1048/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1048<F: Float>(t128: F, t1643: F, t5248: F, t671: F, t3664: F, t9294: F, t11578: F, t11579: F, t1928: F, t11577: F, t11580: F, t561: F, t21643: F, t26561: F, t1743: F, t26597: F) -> (F, F, F, F, F, F) {
    let t34351 = t1643 * t128 * t671 * M_PI * t5248;
    let t34353 = t3664 * t9294;
    let t34356 = t11578 * t11579 * t1928;
    let t34359 = t561 * t11577 * t11580;
    let t34361 = t26561 * t21643;
    let t34363 = t1743 * t26597;
    (t34351, t34353, t34356, t34359, t34361, t34363)
}
