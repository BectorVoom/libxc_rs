//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1693;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta467<F: Float>(t26265: F, t3917: F, t25899: F, t26231: F, t72: F, t7531: F, t686: F, t7284: F, t7289: F, t136: F, t2102: F, t2457: F, t25944: F, t25950: F, t7515: F, t213: F, t7506: F, t2470: F, t7514: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26266, t26268, t26270, t26271, t26272, t26274, t26276, t26277) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1693::<F>(t26265, t3917, t25899, t26231, t72, t7531, t686, t7284, t7289, t136, t2102, t2457);
        let (t26279, t26280, t26282, t26292) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1694::<F>(t25944, t26277, t25950, t7515, t213, t7506, t2470, t7514);
    (t26266, t26268, t26270, t26271, t26272, t26274, t26276, t26277, t26279, t26280, t26282, t26292)
}
