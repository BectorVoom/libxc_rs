//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 733/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk733<F: Float>(t161: F, t4576: F, t1: F, t1368: F, t3: F, t19: F, t545: F, t20: F, t1365: F, t1472: F, t1372: F, t1375: F, t1379: F, t1380: F, t159: F, t39: F, t696: F, t697: F) -> (F, F, F, F) {
    let t4577 = t4576 * t161;
    let t4579 = t1368 * t1;
    let t4580 = t4579 * t3;
    let t4585 = t545 * t19;
    let t4586 = t4585 * t20;
    let t4589 = t1365 * t161;
    let t4592 = t1472 * t161;
    let t4598 = t4577 / F::new(2.0) + F::new(0.9405e-1) * t4580 * t697 - F::new(0.1254e0) * t1372 * t1375 + F::new(0.2358774e-1) * t4586 * t1380 + F::new(0.97533333333333333333e-1) * t696 * t4589 - F::new(0.3145032e-1) * t1379 * t4592 + F::new(0.18830592773509979209e-2) * t159 * t39 * t161;
    (t4577, t4579, t4585, t4598)
}
