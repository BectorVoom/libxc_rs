//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1679;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1680;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1681;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta460(t1426: f64, t545: f64, t2453: f64, t7283: f64, t25920: f64, t7063: f64, t3974: f64, t7259: f64, t2482: f64, t27: f64, t7269: f64, t3981: f64, t2019: f64, t3985: f64, t820: f64, t843: f64, t1416: f64, t3999: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25937, t25944, t25949, t25950, t25969, t25972) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1679(t1426, t545, t2453, t7283, t25920, t7063, t3974, t7259, t2482, t27, t7269);
        let (t25974, t25975, t25978) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1680(t25972, t3981, t2019, t3985, t7269, t820, t843);
        let (t25980, t25981) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1681(t1416, t25978, t3999, t64);
    (t25937, t25944, t25949, t25950, t25969, t25972, t25974, t25975, t25978, t25980, t25981)
}
