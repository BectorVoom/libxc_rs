//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1772;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta403<F: Float>(t17852: F, t460: F, t12050: F, t3603: F, t1284: F, t5216: F, t1204: F, t5477: F, t1269: F, t3781: F, t3766: F, t1770: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17853, t17854, t17861, t17864, t17879, t17880, t17887, t17888, t17934) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1772::<F>(t17852, t460, t12050, t3603, t1284, t5216, t1204, t5477, t1269, t3781, t3766, t1770);
    (t17853, t17854, t17861, t17864, t17879, t17880, t17887, t17888, t17934)
}
