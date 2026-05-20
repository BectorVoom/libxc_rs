//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta846 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2726;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2727;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta846<F: Float>(t17303: F, t5323: F, t12866: F, t5406: F, t58895: F, t17789: F, t21306: F, t17401: F, t17617: F, t15687: F, t17394: F, t3782: F, t17708: F, t59948: F, t370: F, t17727: F, t12916: F, t21258: F, t3718: F, t17753: F, t21045: F, t5401: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t70583, t70612, t70616, t70623, t70629, t70630) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2726::<F>(t17303, t5323, t12866, t5406, t58895, t17789, t21306, t17401, t17617, t15687, t17394, t3782);
        let (t70639, t70646, t70647, t70664, t70667, t70672) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2727::<F>(t17708, t59948, t17394, t370, t17727, t12916, t21258, t3718, t17753, t21045, t12866, t5401, t58895);
    (t70583, t70612, t70616, t70623, t70629, t70630, t70639, t70646, t70647, t70664, t70667, t70672)
}
