//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2287;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta579<F: Float>(t3568: F, t5486: F, t1287: F, t1794: F, t3727: F, t1770: F, t3766: F, t3759: F, t5245: F, t5457: F, t5351: F, t13126: F, t487: F) -> (F, F, F, F, F, F, F) {
        let (t17917, t17921, t17934, t17941, t17944, t17945, t17948) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2287::<F>(t3568, t5486, t1287, t1794, t3727, t1770, t3766, t3759, t5245, t5457, t5351, t13126, t487);
    (t17917, t17921, t17934, t17941, t17944, t17945, t17948)
}
