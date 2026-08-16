//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1522;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta286<F: Float>(t10276: F, t22: F, t2224: F, t588: F, t27: F, t584: F, t20: F, t596: F, t12: F, t583: F, t2231: F, t2237: F, t592: F, t2236: F, t3: F, t25: F, t10271: F, t10273: F, t10275: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10278, t10279, t10280, t10281, t10282, t10284, t10285, t10287, t10288, t10289, t10290) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1522::<F>(t10276, t22, t2224, t588, t27, t584, t20, t596, t12, t583, t2231, t2237, t592);
        let (t10292, t10293, t10295, t10296) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1523::<F>(t10290, t2236, t3, t25, t10271, t10273, t10275, t10278, t10280, t10282, t10284, t10287, t10289);
    (t10278, t10279, t10281, t10284, t10285, t10287, t10288, t10290, t10292, t10293, t10295, t10296)
}
