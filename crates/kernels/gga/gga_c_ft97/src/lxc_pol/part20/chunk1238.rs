//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1238/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1238<F: Float>(t1234: F, t193: F, t25135: F, t6308: F, t852: F, t10683: F, t24980: F, t2682: F, t6318: F, t15175: F, t6317: F, t1636: F, t7087: F, t89: F, t112742: F, t43350: F, t446: F) -> (F, F, F, F, F, F) {
    let t113406 = t6308 * t193 * t852 * t25135 * t1234;
    let t113411 = t24980 * t10683 * t6318 * t1234 * t2682;
    let t113415 = t6317 * t10683 * t6318 * t15175;
    let t113420 = t89 * t1636 * t7087;
    let t113421 = 4.0 / 9.0 * t113420;
    let t113423 = t446 * t43350 * t112742;
    (t113406, t113411, t113415, t113420, t113421, t113423)
}
