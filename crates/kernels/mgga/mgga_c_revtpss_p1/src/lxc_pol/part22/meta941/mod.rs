//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta941 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3176;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta941<F: Float>(t3520: F, t5155: F, t12552: F, t1749: F, t12486: F, t1756: F, t12485: F, t12553: F, t12428: F, t1737: F, t3495: F, t1160: F, t17020: F) -> (F, F, F, F, F, F, F, F) {
        let (t58242, t58247, t58259, t58262, t58300, t58304, t58307, t58310) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3176::<F>(t3520, t5155, t12552, t1749, t12486, t1756, t12485, t12553, t12428, t1737, t3495, t1160, t17020);
    (t58242, t58247, t58259, t58262, t58300, t58304, t58307, t58310)
}
