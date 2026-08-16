//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta962 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta962<F: Float>(t2609: F, t2611: F, t5819: F, t49957: F, t49963: F, t49966: F, t49978: F, t49981: F, t49983: F, t49986: F, t39779: F, t39783: F, t39786: F, t39791: F, t39795: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t61166, t61167, t61168, t61169, t61170, t61171, t61172, t61173, t61174) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3224::<F>(t2609, t2611, t5819, t49957, t49963, t49966, t49978, t49981, t49983, t49986, t39779, t39783, t39786, t39791, t39795);
    (t61166, t61167, t61168, t61169, t61170, t61171, t61172, t61173, t61174)
}
