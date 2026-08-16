//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta226 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk981;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk982;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta226<F: Float>(t12627: F, t487: F, t12295: F, t3566: F, t3754: F, t1209: F, t5462: F, t5477: F, t3634: F, t828: F, t3618: F) -> (F, F, F, F, F, F, F) {
        let (t12628, t12678, t12717, t12751, t12756, t12772) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk981::<F>(t12627, t487, t12295, t3566, t3754, t1209, t5462, t5477, t3634, t828);
        let t12787 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk982::<F>(t3618, t828);
    (t12628, t12678, t12717, t12751, t12756, t12772, t12787)
}
