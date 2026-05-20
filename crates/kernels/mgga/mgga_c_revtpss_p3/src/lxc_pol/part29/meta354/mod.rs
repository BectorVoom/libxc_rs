//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta354<F: Float>(t1207: F, t456: F, t487: F, t1269: F, t3566: F, t1203: F, t3565: F, t3552: F, t1208: F, t3551: F, t1209: F, t3727: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12627, t12628, t12633, t12640, t12641, t12654, t12657, t12658, t12666) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1283::<F>(t1207, t456, t487, t1269, t3566, t1203, t3565, t3552, t1208, t3551, t1209, t3727);
    (t12627, t12628, t12633, t12640, t12641, t12654, t12657, t12658, t12666)
}
