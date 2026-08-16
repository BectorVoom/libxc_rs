//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1255;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1256;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1257;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1258;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1259;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1260;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1261;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1262;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta277(t1539: f64, t6785: f64, t6784: f64, t1599: f64, t1949: f64, t1629: f64, t6800: f64, t6799: f64, t1625: f64, t1948: f64, t345: f64, t1615: f64, t1945: f64, t1060: f64, t383: f64, t7593: f64, t1058: f64, t1610: f64, t1920: f64, t1953: f64, t353: f64, t6687: f64, t6783: f64, t6797: f64, t1055: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7603 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1255(t1539, t6785);
        let t7604 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1256(t6784, t7603);
        let t7607 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1257(t1599, t1949);
        let (t7610, t7611) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1258(t1629, t6800, t6799);
        let t7614 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1259(t1625, t1948);
        let (t7615, t7619) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1260(t345, t7614, t1615, t1945);
        let (t7620, t7622, t7624) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1261(t1060, t7619, t383, t7593, t1058, t1610, t1920, t1953, t353, t6687, t6783, t6797, t7604, t7607, t7611, t7615);
        let t7625 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1262(t1055, t7624);
    (t7603, t7604, t7607, t7610, t7611, t7614, t7615, t7619, t7620, t7622, t7624, t7625)
}
