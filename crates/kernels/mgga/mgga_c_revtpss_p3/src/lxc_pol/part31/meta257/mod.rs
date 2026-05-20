//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1138;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1139;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta257<F: Float>(t7014: F, t780: F, t689: F, t1950: F, t786: F, t789: F, t159: F, t793: F, t218: F, t816: F, t1941: F, t228: F, t802: F, t240: F, t64: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t7015, t7017, t7018, t7020, t7021) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1138::<F>(t7014, t780, t689, t1950, t786, t789, t159, t793);
        let (t7024, t7025) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1139::<F>(t218, t7021, t816, t1941, t228);
        let (t7026, t7028) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1140::<F>(t7025, t802, t240, t64);
    (t7015, t7017, t7018, t7020, t7021, t7024, t7025, t7026, t7028)
}
