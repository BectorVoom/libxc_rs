//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta847 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2985;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta847(t14141: f64, t14143: f64, t4056: f64, t676: f64, t14066: f64, t1432: f64, t686: f64, t72: f64, t14188: f64, t2439: f64, t2777: f64, t10073: f64, t14129: f64, t14159: f64, t3964: f64, t9285: f64, t213: f64, t225: f64, t46475: f64, t10019: f64, t14114: f64, t14145: f64, t2482: f64, t4114: f64, t5658: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49403, t49407, t49426, t49429) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2985(t14141, t14143, t4056, t676, t14066, t1432, t686, t72, t14188, t2439, t2777, t10073, t14129);
        let (t49432, t49439, t49446, t49450) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2986(t14159, t3964, t9285, t213, t225, t46475, t10019, t14114, t14145, t2482, t4114, t5658);
    (t49403, t49407, t49426, t49429, t49432, t49439, t49446, t49450)
}
