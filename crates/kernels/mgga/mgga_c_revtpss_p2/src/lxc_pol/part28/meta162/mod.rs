//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta162 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk849;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk850;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk851;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta162<F: Float>(t1198: F, t3531: F, t1188: F, t3495: F, t3497: F, t1196: F, t1179: F, t3515: F, t3520: F, t3523: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F, t459: F, t1203: F, t1208: F, t487: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3533, t3535, t3537, t3539, t3541, t3543, t3545, t3546, t3551) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk849::<F>(t1198, t3531, t1188, t3495, t3497, t1196, t1179, t3515, t3520, t3523, t3356, t3358, t3365, t3370, t3374);
        let t3552 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk850::<F>(t3551, t459);
        let t3555 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk851::<F>(t1203, t1208);
        let t3556 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk852::<F>(t3555, t487);
    (t3533, t3535, t3537, t3539, t3541, t3543, t3545, t3546, t3551, t3552, t3555, t3556)
}
