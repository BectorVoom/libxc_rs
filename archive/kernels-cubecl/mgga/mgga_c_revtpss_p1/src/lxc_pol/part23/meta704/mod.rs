//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta704 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2454;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2455;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta704<F: Float>(t72: F, t9940: F, t245: F, t3951: F, t3964: F, t9732: F, t1353: F, t9994: F, t136: F, t4010: F, t220: F, t2482: F, t27: F, t9991: F, t1389: F, t40604: F, t10111: F, t22: F, t4092: F, t39515: F, t4083: F, t10043: F, t9303: F, t14192: F, t555: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47247, t47248, t47262, t47264, t47273, t47274, t47293) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2454::<F>(t72, t9940, t245, t3951, t3964, t9732, t1353, t9994, t136, t4010, t220, t2482, t27, t9991);
        let (t47337, t47348, t47351, t47352, t47371) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2455::<F>(t1389, t3964, t40604, t10111, t22, t4092, t39515, t4083, t10043, t9303, t14192, t555);
    (t47247, t47248, t47262, t47264, t47273, t47274, t47293, t47337, t47348, t47351, t47352, t47371)
}
