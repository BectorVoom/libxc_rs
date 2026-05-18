//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 92/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk92<F: Float>(t40: F, t117: F, zeta_threshold: F) -> F {
    let t225 = F::new(2.0) <= zeta_threshold;
    let t228 = F::new(0.0) <= zeta_threshold;
    let t278 = t40 * t40;
    let t279 = piecewise3::<f64>(t225, t117, t278);
    let t280 = piecewise3::<f64>(t228, t117, F::new(0.0));
    let t282 = t279 / F::new(2.0) + t280 / F::new(2.0);
    t282
}
