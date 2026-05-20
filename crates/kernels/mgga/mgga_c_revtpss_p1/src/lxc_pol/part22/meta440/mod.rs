//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2077;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2078;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta440<F: Float>(t10868: F, t241: F, t820: F, t14547: F, t4364: F, t4365: F, t2724: F, t2747: F, t4450: F, t14676: F, t4366: F, t10811: F, t4452: F, t2754: F, t231: F, t2394: F, t10770: F, t2719: F, t844: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t14894 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2077::<F>(t10868, t241, t820);
        let (t14896, t14900, t14904, t14907, t14910, t14914, t14917) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2078::<F>(t14547, t4364, t4365, t2724, t2747, t4450, t14676, t4366, t10811, t4452, t2754, t231, t2394);
        let (t14919, t14923) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2079::<F>(t10770, t14917, t4365, t2719, t820, t844);
    (t14894, t14896, t14900, t14904, t14907, t14910, t14914, t14917, t14919, t14923)
}
