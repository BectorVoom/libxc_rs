//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1793;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1794;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta493(t25944: f64, t25946: f64, t1426: f64, t25920: f64, t7063: f64, t7286: f64, t2470: f64, t7285: f64, t7289: f64, t3974: f64, t7259: f64, t2482: f64, t27: f64, t7269: f64, t3981: f64, t2019: f64, t3985: f64, t820: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25948, t25949, t25950, t25951, t25953) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1793(t25944, t25946, t1426, t25920, t7063, t7286, t2470, t7285);
        let (t25955, t25970, t25972) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1794(t25953, t7289, t3974, t7259, t2482, t27, t7269);
        let (t25973, t25976, t25978) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1795(t25972, t3981, t2019, t3985, t7269, t820, t843);
    (t25948, t25949, t25950, t25951, t25953, t25955, t25970, t25972, t25973, t25976, t25978)
}
