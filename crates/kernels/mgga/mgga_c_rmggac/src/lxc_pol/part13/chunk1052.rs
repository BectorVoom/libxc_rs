//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1052/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1052<F: Float>(t39338: F, t34960: F, t37297: F, t39330: F, t39333: F, t39341: F, t39345: F, t39350: F, t39355: F, t39360: F, t39362: F, t39367: F, t39370: F, t39374: F, t39379: F, t39384: F, t5928: F, t8042: F) -> F {
    let t42928 = F::new(0.60975299583150056624e-3) * t39338;
    let t42944 = F::new(0.212822999466489197e-4) * t39330 + F::new(0.162600798888400151e-2) * t39333 - t42928 + F::new(0.68400385060046895e-6) * t39341 + F::new(0.68400385060046895e-6) * t39345 + F::new(0.1702583995731913576e-4) * t39350 - F::new(0.2553875993597870364e-4) * t39355 + F::new(0.638468998399467591e-4) * t39360 - F::new(0.68186654135613354325e-2) * t39362 - F::new(0.5854073720911195298e0) * t34960 - F::new(0.2363e1) * t37297 + F::new(0.60975299583150056624e-3) * t39367 - F::new(0.16163010989689081288e-5) * t39370 + F::new(0.13637330827122670865e-1) * t39374 + F::new(0.15323255961587222184e-3) * t39379 - F::new(0.20431007948782962912e-3) * t39384 - F::new(0.11974241701863808564e0) * t5928 * t8042;
    t42944
}
