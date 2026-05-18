//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 86/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk86<F: Float>(t218: F, t220: F, t36: F, t217: F, t43: F, t40: F, zeta_threshold: F) -> (F, F) {
    let t219 = t218 <= zeta_threshold;
    let t222 = piecewise3::<f64>(t219, t36, t220 * t218);
    let t224 = (t217 + t222 - F::new(2.0)) * t43;
    let t225 = F::new(2.0) <= zeta_threshold;
    let t227 = piecewise3::<f64>(t225, t36, F::new(2.0) * t40);
    let t228 = F::new(0.0) <= zeta_threshold;
    let t229 = piecewise3::<f64>(t228, t36, F::new(0.0));
    let t231 = (t227 + t229 - F::new(2.0)) * t43;
    (t224, t231)
}
