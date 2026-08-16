//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1193/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1193<F: Float>(t31270: F, t31272: F, t31274: F, t31277: F, t31279: F, t31282: F, t31284: F, t31287: F, t31923: F, t31937: F, t31940: F, t31942: F, t31944: F, t577: F, t671: F, t8508: F) -> F {
    let t31949 = F::cast_from(0.45e1_f64) * t31923 * t577 + F::cast_from(0.135e2_f64) * t31937 * t671 + F::cast_from(0.135e2_f64) * t31940 + F::cast_from(27.0_f64) * t31942 + F::cast_from(0.135e2_f64) * t31944 + F::cast_from(0.135e2_f64) * t31270 + F::cast_from(27.0_f64) * t31272 + F::cast_from(0.135e2_f64) * t31274 + t31277 + t31279 + t31282 + t31284 + t31287 + t8508;
    t31949
}
