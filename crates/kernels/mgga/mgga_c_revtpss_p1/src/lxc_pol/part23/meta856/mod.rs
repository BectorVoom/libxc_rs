//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta856 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2745;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta856<F: Float>(t17708: F, t59498: F, t12916: F, t21041: F, t3718: F, t21165: F, t12809: F, t20796: F, t13045: F, t5284: F, t5245: F, t5457: F, t1209: F, t1284: F, t6695: F, t20849: F, t3754: F, t3781: F, t6564: F, t20800: F, t3302: F, t13141: F, t1811: F, t460: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t72011, t72017, t72064, t72071, t72086, t72143) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2745::<F>(t17708, t59498, t12916, t21041, t3718, t21165, t12809, t20796, t13045, t5284, t5245, t5457);
        let (t72267, t72270, t72326, t72329, t72343) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2746::<F>(t1209, t1284, t6695, t20849, t3754, t3781, t6564, t20800, t3302, t13141, t1811, t460);
    (t72011, t72017, t72064, t72071, t72086, t72143, t72267, t72270, t72326, t72329, t72343)
}
