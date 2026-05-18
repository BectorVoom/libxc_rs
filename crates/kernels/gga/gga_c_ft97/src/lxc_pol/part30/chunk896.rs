//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 896/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk896<F: Float>(t299: F, t36047: F, t36275: F, t332: F, t113: F, t1275: F, t34341: F, t5: F, t7692: F, t992: F, t2: F, t7242: F, t14: F, t7469: F) -> (F, F, F, F, F) {
    let t300 = F::new(10000000.0) <= t299;
    let t36276 = t36047 + t36275;
    let t36277 = t36276 * t332;
    let t36285 = piecewise3::<f64>(t300, F::new(0.0), t5 * t36277 * t113 / F::new(4.0) + t5 * t7692 * t992 / F::new(4.0) + t34341 * t1275 / F::new(4.0));
    let t36452 = t7242 * t2;
    let t36791 = t7469 * t14;
    (t36276, t36277, t36285, t36452, t36791)
}
