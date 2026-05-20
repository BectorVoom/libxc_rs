//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta775 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta775<F: Float>(t58225: F, t1744: F, t3477: F, t3520: F, t5155: F, t12552: F, t1749: F, t12486: F, t1756: F, t12485: F, t12428: F, t1737: F) -> (F, F, F, F, F, F, F) {
        let (t58226, t58237, t58242, t58247, t58259, t58262, t58304) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2579::<F>(t58225, t1744, t3477, t3520, t5155, t12552, t1749, t12486, t1756, t12485, t12428, t1737);
    (t58226, t58237, t58242, t58247, t58259, t58262, t58304)
}
