//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 972/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk972<F: Float>(t9274: F, t9271: F, t9292: F, t9353: F, t9357: F, t9360: F, t9368: F, t9370: F, t9375: F, t9377: F, t9380: F, t9382: F, t9426: F, t957: F, t938: F, t3491: F, t937: F) -> (F, F, F, F, F) {
    let t9433 = 0.59793333333333333334e0 * t9274;
    let t9439 = 0.49294e0 * t9353 + 0.24647e0 * t9357 + 0.27385555555555555555e0 * t9360 + 0.39862222222222222223e0 * t9271 + 0.1898925e1 * t9368 + 0.3071625e0 * t9370 - t9433 + 0.8969e0 * t9292 - 0.1898925e1 * t9375 - 0.9494625e0 * t9377 + 0.3071625e0 * t9380 + 0.15358125e0 * t9382;
    let t9440 = t9426 + t9439;
    let t9441 = t9440 * t957;
    let t9443 = 1.0 * t938 * t9441;
    let t9444 = t3491 * t937;
    (t9433, t9440, t9441, t9443, t9444)
}
