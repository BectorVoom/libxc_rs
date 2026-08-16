//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta214 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk837;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk838;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta214(t1873: f64, t3957: f64, t1353: f64, t1872: f64, t800: f64, t124: f64, t5591: f64, t3938: f64, t5674: f64, t3936: f64, t1399: f64, t5673: f64, t125: f64, t1868: f64, t1370: f64, t3934: f64, t3944: f64, t3950: f64, t3953: f64, t3958: f64, t3967: f64, t3976: f64, t3982: f64, t3987: f64, t3990: f64, t3996: f64, t5680: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5681, t5686, t5690, t5697, t5701) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk837(t1873, t3957, t1353, t1872, t800, t124, t5591, t3938, t5674, t3936, t1399, t5673);
        let (t5706, t5709) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk838(t125, t1868, t1399, t3936, t1370, t3934, t3944, t3950, t3953, t3958, t3967, t3976, t3982, t3987, t3990, t3996, t5681, t5686, t5690, t5697, t5701);
        let t5710 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk839(t5680, t5709);
    (t5681, t5686, t5690, t5697, t5701, t5706, t5710)
}
