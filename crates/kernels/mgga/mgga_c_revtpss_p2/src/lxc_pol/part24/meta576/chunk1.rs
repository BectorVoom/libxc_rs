//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1763/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1763<F: Float>(t81230: F, t81232: F, t81234: F, t81425: F, t81427: F, t81429: F, t89828: F, t89843: F, t89847: F, t89855: F, t90459: F, t90464: F, t90470: F, t90473: F) -> F {
    let t90558 = -F::cast_from(0.71752000000000000001e1_f64) * t89828 + F::cast_from(0.1898925e1_f64) * t90459 - F::cast_from(0.21908444444444444444e0_f64) * t81425 + F::cast_from(0.43816888888888888888e0_f64) * t81427 - F::cast_from(0.13145066666666666666e1_f64) * t81429 + F::cast_from(0.46074375e0_f64) * t90464 - F::cast_from(0.79724444444444444444e0_f64) * t89843 + F::cast_from(0.107628e2_f64) * t89847 + F::cast_from(0.23917333333333333333e1_f64) * t89855 - F::cast_from(0.10954222222222222222e0_f64) * t90470 - F::cast_from(0.98587999999999999999e0_f64) * t90473 - F::cast_from(0.44291358024691358024e0_f64) * t81230 + F::cast_from(0.15944888888888888889e1_f64) * t81232 - F::cast_from(0.23917333333333333333e1_f64) * t81234;
    t90558
}
