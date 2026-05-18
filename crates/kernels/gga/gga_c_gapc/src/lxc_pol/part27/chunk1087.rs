//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1087/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1087<F: Float>(t11965: F, t28353: F, t871: F, t2511: F, t3757: F, t1026: F, t2786: F, t3304: F, t3772: F, t9913: F, t11781: F, t9485: F) -> (F, F, F, F, F) {
    let t33452 = t871 * t11965 * t28353;
    let t33454 = t3757 * t2511;
    let t33457 = t2786 * t1026 * t3304;
    let t33460 = t3772 * t9913;
    let t33462 = t11781 * t9485;
    (t33452, t33454, t33457, t33460, t33462)
}
