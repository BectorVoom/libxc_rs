//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1874;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1875;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta495(t25972: f64, t3981: f64, t2019: f64, t3985: f64, t7269: f64, t820: f64, t843: f64, t1416: f64, t3999: f64, t64: f64, t239: f64, t4006: f64, t240: f64, t7262: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25973, t25974, t25976, t25978) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1874(t25972, t3981, t2019, t3985, t7269, t820, t843);
        let (t25979, t25980, t25981) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1875(t1416, t25978, t3999, t64);
        let (t25984, t25986) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1876(t239, t25981, t820, t4006, t240, t7262);
    (t25973, t25974, t25976, t25978, t25979, t25980, t25981, t25984, t25986)
}
