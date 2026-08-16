//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1894;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1895;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta504(t1937: f64, t27123: f64, t4292: f64, t94: f64, t6993: f64, t7732: f64, t7003: f64, t2322: f64, t7735: f64, t4254: f64, t1936: f64, t5517: f64, t651: f64, t1843: f64, t1932: f64, t27116: f64, t27118: f64, t27120: f64, t27122: f64, t6983: f64, t7746: f64, t1518: f64, t7221: f64, t7235: f64, t7935: f64, t1353: f64, t1907: f64, t8717: f64, t25082: f64, t1962: f64, t198: f64, t205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27125, t27126, t27128, t27130, t27132, t27134, t27136, t27137) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1894(t1937, t27123, t4292, t94, t6993, t7732, t7003, t2322, t7735, t4254, t1936, t5517);
        let t27142 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1895(t27137, t651, t1843, t1932, t2322, t27116, t27118, t27120, t27122, t27125, t27128, t27130, t27132, t27134, t27136, t5517, t6983, t7746);
        let (t27145, t27152, t27153, t27154, t27156, t27158) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1896(t1518, t7221, t7235, t7935, t1353, t1907, t8717, t25082, t1962, t198, t205);
    (t27126, t27137, t27142, t27145, t27152, t27153, t27154, t27156, t27158)
}
