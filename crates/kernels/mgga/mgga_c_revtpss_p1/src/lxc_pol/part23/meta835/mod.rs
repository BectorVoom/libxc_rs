//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta835 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2706;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta835<F: Float>(t5219: F, t5412: F, t1284: F, t21333: F, t20382: F, t3520: F, t3383: F, t6433: F, t1130: F, t20469: F, t3432: F, t1179: F, t20567: F) -> (F, F, F, F, F, F, F) {
        let (t68658, t68674, t68680, t68792, t68947, t68952, t69354) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2706::<F>(t5219, t5412, t1284, t21333, t20382, t3520, t3383, t6433, t1130, t20469, t3432, t1179, t20567);
    (t68658, t68674, t68680, t68792, t68947, t68952, t69354)
}
