//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1281;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta294<F: Float>(t225: F, t9801: F, t4062: F, t1386: F, t2482: F, t814: F, t136: F, t1412: F, t220: F, t1353: F, t4003: F, t2735: F, t4086: F, t3994: F, t808: F, t521: F, t9342: F, t14: F, t588: F, t2496: F, t4038: F, t123: F, t1330: F, t2630: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9802, t9804, t9816, t9817, t9818, t9835, t9845) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1281::<F>(t225, t9801, t4062, t1386, t2482, t814, t136, t1412, t220, t1353, t4003, t2735, t4086);
        let (t9847, t9854, t9856, t9858, t9861) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1282::<F>(t3994, t808, t9845, t521, t9342, t14, t588, t2496, t4038, t123, t1330, t2630);
    (t9802, t9804, t9816, t9817, t9818, t9835, t9845, t9847, t9854, t9856, t9858, t9861)
}
