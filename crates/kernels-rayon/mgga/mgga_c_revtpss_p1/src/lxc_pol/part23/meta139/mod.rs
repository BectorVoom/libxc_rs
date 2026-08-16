//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta139 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk894;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk895;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk896;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta139(t1376: f64, t2689: f64, t1353: f64, t1413: f64, t547: f64, t807: f64, t2700: f64, t535: f64, t1369: f64, t794: f64, t1372: f64, t2453: f64, t546: f64, t1389: f64, t2713: f64, t2668: f64, t550: f64, t816: f64, t1379: f64, t1408: f64, t2482: f64, t27: f64, t136: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3950, t3951, t3952, t3953, t3956, t3957) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk894(t1376, t2689, t1353, t1413, t547, t807, t2700, t535, t1369, t794);
        let (t3958, t3964) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk895(t1372, t3957, t2453, t546);
        let (t3967, t3976, t3978) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk896(t1389, t2713, t3964, t2668, t550, t816, t1379, t1408, t2482, t27);
        let t3979 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk897(t136, t1413);
    (t3950, t3951, t3952, t3953, t3956, t3957, t3958, t3964, t3967, t3976, t3978, t3979)
}
