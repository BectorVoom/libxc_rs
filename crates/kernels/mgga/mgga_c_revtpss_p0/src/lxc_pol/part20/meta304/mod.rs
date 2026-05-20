//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1192;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta304<F: Float>(t3519: F, t444: F, t439: F, t1187: F, t3497: F, t3523: F, t1175: F, t3495: F, t1188: F, t1189: F, t3515: F, t1170: F, t3471: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t12485 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1192::<F>(t3519, t444);
        let (t12486, t12487, t12488, t12491, t12494, t12497, t12500, t12501, t12504) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1193::<F>(t12485, t439, t1187, t3497, t3523, t1175, t3495, t1188, t1189, t3515, t1170, t3471);
    (t12485, t12486, t12487, t12488, t12491, t12494, t12497, t12500, t12501, t12504)
}
