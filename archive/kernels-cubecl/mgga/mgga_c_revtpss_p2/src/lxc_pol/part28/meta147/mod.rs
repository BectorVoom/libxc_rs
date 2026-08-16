//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta147 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk795;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk796;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta147<F: Float>(t1025: F, t3215: F, t3075: F, t373: F, t371: F, t372: F, t225: F, t3046: F, t366: F, t362: F, t40: F, t611: F, t361: F, t351: F, t1054: F, t1058: F, t1014: F, t2857: F, t2251: F, t1012: F, t1010: F, t614: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3216, t3218, t3220, t3223) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk795::<F>(t1025, t3215, t3075, t373, t371, t372, t225, t3046);
        let t3224 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk796::<F>(t3223, t366);
        let (t3229, t3230, t3231, t3234, t3237, t3238, t3241) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk797::<F>(t362, t40, t611, t361, t351, t1054, t1058, t1014, t2857, t2251, t1012, t1010, t614);
    (t3216, t3218, t3220, t3223, t3224, t3229, t3230, t3231, t3234, t3237, t3238, t3241)
}
