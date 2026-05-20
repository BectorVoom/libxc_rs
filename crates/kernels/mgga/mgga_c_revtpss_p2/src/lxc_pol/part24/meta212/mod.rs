//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta212 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk954;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk955;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta212<F: Float>(t992: F, t338: F, t378: F, t1031: F, t342: F, t3145: F, t334: F, t368: F, t365: F, t3144: F, t3153: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11198, t11199, t11200) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk954::<F>(t992, t338);
        let (t11201, t11238, t11239) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk955::<F>(t11200, t378, t1031);
        let (t11240, t11243, t11244, t11245, t11246, t11249) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk956::<F>(t11239, t342, t3145, t334, t368, t365, t3144, t3153, t73);
    (t11198, t11199, t11200, t11201, t11238, t11239, t11240, t11243, t11244, t11245, t11246, t11249)
}
