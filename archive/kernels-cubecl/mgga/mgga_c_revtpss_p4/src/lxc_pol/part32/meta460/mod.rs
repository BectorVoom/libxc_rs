//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1679;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1680;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1681;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta460<F: Float>(t1426: F, t545: F, t2453: F, t7283: F, t25920: F, t7063: F, t3974: F, t7259: F, t2482: F, t27: F, t7269: F, t3981: F, t2019: F, t3985: F, t820: F, t843: F, t1416: F, t3999: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25937, t25944, t25949, t25950, t25969, t25972) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1679::<F>(t1426, t545, t2453, t7283, t25920, t7063, t3974, t7259, t2482, t27, t7269);
        let (t25974, t25975, t25978) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1680::<F>(t25972, t3981, t2019, t3985, t7269, t820, t843);
        let (t25980, t25981) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1681::<F>(t1416, t25978, t3999, t64);
    (t25937, t25944, t25949, t25950, t25969, t25972, t25974, t25975, t25978, t25980, t25981)
}
