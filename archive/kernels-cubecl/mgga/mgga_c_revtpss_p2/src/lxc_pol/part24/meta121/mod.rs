//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta121 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk658;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta121<F: Float>(t4398: F, t762: F, t162: F, t2611: F, t227: F, t73: F, t1544: F, t853: F, t1559: F, t221: F, t2485: F, t2484: F) -> (F, F, F, F, F, F) {
        let (t4399, t4401, t4415, t4416, t4430, t4431) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk658::<F>(t4398, t762, t162, t2611, t227, t73, t1544, t853, t1559, t221, t2485, t2484);
    (t4399, t4401, t4415, t4416, t4430, t4431)
}
