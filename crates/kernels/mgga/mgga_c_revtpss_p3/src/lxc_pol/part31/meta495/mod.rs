//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1806;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1807;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1808;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta495<F: Float>(t25944: F, t25946: F, t1426: F, t25920: F, t7063: F, t7286: F, t2470: F, t7285: F, t7289: F, t3974: F, t7259: F, t2482: F, t27: F, t7269: F, t3981: F, t2019: F, t3985: F, t820: F, t843: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25948, t25949, t25950, t25951, t25953) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1806::<F>(t25944, t25946, t1426, t25920, t7063, t7286, t2470, t7285);
        let (t25955, t25970, t25972) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1807::<F>(t25953, t7289, t3974, t7259, t2482, t27, t7269);
        let (t25973, t25976, t25978) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1808::<F>(t25972, t3981, t2019, t3985, t7269, t820, t843);
    (t25948, t25949, t25950, t25951, t25953, t25955, t25970, t25972, t25973, t25976, t25978)
}
