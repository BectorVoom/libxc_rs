//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2010;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2011;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta410<F: Float>(t14224: F, t4100: F, t2782: F, t10014: F, t5741: F, t13790: F, t1398: F, t10022: F, t10066: F, t10070: F, t10074: F, t10080: F, t10085: F, t10098: F, t10102: F, t14066: F, t14203: F, t14209: F, t14218: F, t14221: F, t213: F, t546: F, t1892: F, t4086: F, t786: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14225, t14227, t14229, t14230) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2010::<F>(t14224, t4100, t2782, t10014, t5741, t13790, t1398);
        let (t14231, t14233, t14237) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2011::<F>(t10022, t14230, t2782, t10066, t10070, t10074, t10080, t10085, t10098, t10102, t14066, t14203, t14209, t14218, t14221, t14227, t14229, t213, t546);
        let (t14238, t14239) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2012::<F>(t1892, t4086, t786);
    (t14225, t14227, t14229, t14230, t14231, t14233, t14237, t14238, t14239)
}
