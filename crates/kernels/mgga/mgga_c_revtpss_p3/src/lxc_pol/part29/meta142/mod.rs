//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta142 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk737;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk738;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk739;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk740;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta142<F: Float>(t3109: F, t906: F, t247: F, t1063: F, t1086: F, t994: F, t3090: F, t373: F, t66: F, t828: F, t1043: F, t999: F, t1045: F, t1032: F, t989: F, t1040: F, t1024: F, t1062: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3111, t3112, t3114, t3115) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk737::<F>(t3109, t906, t247, t1063, t1086, t994, t3090);
        let t3116 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk738::<F>(t373, t66);
        let t3117 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk739::<F>(t3116, t828);
        let (t3118, t3119, t3120, t3123, t3124) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk740::<F>(t1043, t999, t1045, t3117, t1032, t989, t1040);
        let t3127 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk741::<F>(t1024, t1062);
    (t3111, t3112, t3114, t3115, t3116, t3117, t3118, t3119, t3120, t3123, t3124, t3127)
}
