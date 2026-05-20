//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2092;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2093;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta637<F: Float>(t15654: F, t1976: F, t27708: F, t3336: F, t11108: F, t7840: F, t33: F, t41154: F, t1711: F, t2411: F, t28150: F, t6973: F, t1497: F, t6977: F, t1926: F, t1927: F, t4241: F, t25163: F, t7715: F, t644: F, t7719: F, t13272: F, t607: F, t10301: F, t1470: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t100760, t100802, t100806, t100981, t100987, t101211) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2092::<F>(t15654, t1976, t27708, t3336, t11108, t7840, t33, t41154, t1711, t2411, t28150, t6973);
        let (t101215, t101219, t101222, t101227, t101230, t101237) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2093::<F>(t1497, t6977, t1926, t1927, t4241, t25163, t7715, t644, t7719, t13272, t607, t10301, t1470);
    (t100760, t100802, t100806, t100981, t100987, t101211, t101215, t101219, t101222, t101227, t101230, t101237)
}
