//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1001/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1001<F: Float>(t2394: F, t3750: F, t9624: F, t3388: F, t33906: F, t2494: F, t3757: F, t33338: F, t3781: F, t17819: F, t2578: F, t3679: F, t11837: F, t612: F, t761: F, t11918: F, t29473: F) -> (F, F, F, F, F, F, F) {
    let t34205 = t2394 * t3750 * t9624;
    let t34207 = t33906 * t3388;
    let t34209 = t3757 * t2494;
    let t34211 = t33338 * t3781;
    let t34214 = t2578 * t3679 * t17819;
    let t34217 = t761 * t612 * t11837;
    let t34219 = t11918 * t29473;
    (t34205, t34207, t34209, t34211, t34214, t34217, t34219)
}
