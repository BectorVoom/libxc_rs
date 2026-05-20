//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1894;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1895;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta504<F: Float>(t1937: F, t27123: F, t4292: F, t94: F, t6993: F, t7732: F, t7003: F, t2322: F, t7735: F, t4254: F, t1936: F, t5517: F, t651: F, t1843: F, t1932: F, t27116: F, t27118: F, t27120: F, t27122: F, t6983: F, t7746: F, t1518: F, t7221: F, t7235: F, t7935: F, t1353: F, t1907: F, t8717: F, t25082: F, t1962: F, t198: F, t205: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t27125, t27126, t27128, t27130, t27132, t27134, t27136, t27137) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1894::<F>(t1937, t27123, t4292, t94, t6993, t7732, t7003, t2322, t7735, t4254, t1936, t5517);
        let t27142 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1895::<F>(t27137, t651, t1843, t1932, t2322, t27116, t27118, t27120, t27122, t27125, t27128, t27130, t27132, t27134, t27136, t5517, t6983, t7746);
        let (t27145, t27152, t27153, t27154, t27156, t27158) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1896::<F>(t1518, t7221, t7235, t7935, t1353, t1907, t8717, t25082, t1962, t198, t205);
    (t27126, t27137, t27142, t27145, t27152, t27153, t27154, t27156, t27158)
}
