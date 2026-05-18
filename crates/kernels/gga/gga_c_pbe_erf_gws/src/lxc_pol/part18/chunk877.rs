//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 877/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk877<F: Float>(t367: F, t6553: F, t899: F, t1112: F, t4394: F, t3253: F, t6203: F, t1154: F, t6455: F, t3261: F, t6416: F, t3291: F) -> (F, F, F, F, F, F) {
    let t9425 = t899 * t6553 * t367;
    let t9441 = t1112 * t4394;
    let t9447 = F::new(7.0) / F::new(288.0) * t6203 * t3253;
    let t9457 = t6455 * t1154;
    let t9464 = F::new(7.0) / F::new(576.0) * t6416 * t3261;
    let t9474 = F::new(7.0) / F::new(1152.0) * t6416 * t3291;
    (t9425, t9441, t9447, t9457, t9464, t9474)
}
