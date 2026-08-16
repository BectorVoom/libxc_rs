//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta251 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1436;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1437;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta251<F: Float>(t3869: F, t9575: F, t1331: F, t3860: F, t186: F, t685: F, t793: F, t1337: F, t4146: F, t565: F, t1333: F, t30: F, t513: F, t33: F, t516: F, t2435: F, t3900: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9577, t9578, t9586) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1436::<F>(t3869, t9575, t1331, t3860, t186, t685, t793);
        let (t9588, t9593, t9597, t9598, t9603, t9605, t9615, t9617, t9632) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1437::<F>(t1337, t9586, t4146, t565, t1333, t3860, t30, t513, t33, t516, t2435, t3900);
    (t9577, t9578, t9586, t9588, t9593, t9597, t9598, t9603, t9605, t9615, t9617, t9632)
}
