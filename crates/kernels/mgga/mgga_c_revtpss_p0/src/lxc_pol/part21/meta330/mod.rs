//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1635;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1636;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta330<F: Float>(t2258: F, t606: F, t4801: F, t1042: F, t1031: F, t342: F, t3145: F, t334: F, t368: F, t365: F, t3144: F, t1043: F, t3151: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t11231 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1635::<F>(t2258, t606);
        let (t11232, t11233, t11238, t11239) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1636::<F>(t11231, t4801, t1042, t1031);
        let (t11240, t11243, t11244, t11245, t11246, t11247) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1637::<F>(t11239, t342, t3145, t334, t368, t365, t3144, t1043, t3151);
    (t11231, t11232, t11233, t11238, t11239, t11240, t11243, t11244, t11245, t11246, t11247)
}
