//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1693;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta467(t26265: f64, t3917: f64, t25899: f64, t26231: f64, t72: f64, t7531: f64, t686: f64, t7284: f64, t7289: f64, t136: f64, t2102: f64, t2457: f64, t25944: f64, t25950: f64, t7515: f64, t213: f64, t7506: f64, t2470: f64, t7514: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26266, t26268, t26270, t26271, t26272, t26274, t26276, t26277) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1693(t26265, t3917, t25899, t26231, t72, t7531, t686, t7284, t7289, t136, t2102, t2457);
        let (t26279, t26280, t26282, t26292) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1694(t25944, t26277, t25950, t7515, t213, t7506, t2470, t7514);
    (t26266, t26268, t26270, t26271, t26272, t26274, t26276, t26277, t26279, t26280, t26282, t26292)
}
