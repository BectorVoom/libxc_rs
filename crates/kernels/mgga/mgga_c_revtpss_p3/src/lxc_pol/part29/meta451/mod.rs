//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1689;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1690;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1691;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta451<F: Float>(t1398: F, t1444: F, t543: F, t1426: F, t545: F, t2453: F, t7283: F, t25920: F, t7063: F, t3974: F, t7259: F, t2482: F, t27: F, t7269: F, t3981: F, t2019: F, t3985: F, t820: F, t843: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25933, t25937, t25944, t25949) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1689::<F>(t1398, t1444, t543, t1426, t545, t2453, t7283, t25920);
        let (t25950, t25969, t25972) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1690::<F>(t25949, t7063, t3974, t7259, t2482, t27, t7269);
        let (t25973, t25974, t25975, t25978) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1691::<F>(t25972, t3981, t2019, t3985, t7269, t820, t843);
    (t25933, t25937, t25944, t25949, t25950, t25969, t25972, t25973, t25974, t25975, t25978)
}
