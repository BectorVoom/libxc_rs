//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 897/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk897<F: Float>(t2179: F, t35196: F, t144: F, t1017: F, t7400: F, t574: F, t1053: F, t9439: F, t34956: F, t34952: F, t34954: F, t1901: F, t33215: F, t33218: F, t35181: F, t35185: F, t35189: F, t35193: F, t446: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t35197 = t2179 * t35196;
    let t35198 = t144 * t35197;
    let t35201 = t7400 * t1017;
    let t35203 = t574 * t2179 * t35201;
    let t35206 = t7400 * t1053;
    let t35207 = t9439 * t35206;
    let t35208 = t144 * t35207;
    let t35211 = t144 * t34956;
    let t35214 = t144 * t34952;
    let t35217 = t144 * t34954;
    let t35220 = -t446 * t35181 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t446 * t35185 - t446 * t35189 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t1901 * t35193 + F::new(2.0) / F::new(3.0) * t446 * t35198 - F::new(2.0) / F::new(3.0) * t446 * t35203 - F::new(2.0) * t446 * t35208 - F::new(2.0) / F::new(3.0) * t446 * t35211 + t33215 - t33218 - F::new(2.0) / F::new(3.0) * t446 * t35214 - t446 * t35217 / F::new(3.0);
    (t35197, t35198, t35201, t35203, t35206, t35207, t35208, t35211, t35214, t35217, t35220)
}
