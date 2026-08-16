//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1974;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1975;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1976;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1977;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1978;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1979;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta564(t265: f64, t504: f64, t27421: f64, t27757: f64, t27797: f64, t27832: f64, t3640: f64, t8090: f64, t1254: f64, t1763: f64, t1256: f64, t193: f64, t24905: f64, t24909: f64, t25882: f64, t336: f64, t4700: f64, t5091: f64, t7398: f64, t28: f64, t1409: f64, t2161: f64, t25949: f64, t3966: f64, t52: f64, t607: f64, t7402: f64, t8097: f64, t27380: f64, t113: f64, t24988: f64, t24989: f64, t24993: f64, t24998: f64, t25005: f64, t25007: f64, t25011: f64, t25969: f64, t25973: f64, t27290: f64, t27293: f64, t27371: f64, t510: f64, t650: f64, t652: f64, t8103: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t111: f64, t7982: f64, t1442: f64, t1774: f64, t2114: f64, t25975: f64, t25977: f64, t25979: f64, t25982: f64, t25987: f64, t25991: f64, t25993: f64, t25996: f64, t25998: f64, t26002: f64, t26005: f64, t5107: f64, t672: f64, t7264: f64, t7408: f64, t1266: f64, t2165: f64, t2167: f64, t2314: f64, t26006: f64, t26141: f64, t26144: f64, t26145: f64, t26147: f64, t26150: f64, t26153: f64, t26157: f64, t4026: f64, t4028: f64, t4034: f64, t5361: f64, t7271: f64, t7983: f64, t7989: f64, t1458: f64, t2113: f64, t671: f64, t24932: f64, t26109: f64, t26111: f64, t26113: f64, t26116: f64, t26119: f64, t26121: f64, t26123: f64, t26125: f64, t26137: f64, t4072: f64, t7266: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27834, t27838, t27843, t27850) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1974(t265, t504, t27421, t27757, t27797, t27832, t3640, t8090, t1254, t1763, t1256, t193, t24905, t24909, t25882, t336, t4700, t5091, t7398);
        let (t27858, t27860) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1975(t28, t1409, t2161, t25949, t27850, t3966, t52, t607, t7402, t8097, t27380, t113, t24988, t24989, t24993, t24998, t25005, t25007, t25011, t25969, t25973, t27290, t27293, t27371, t510, t650, t652, t8103, dens_threshold, rho1, zeta_threshold);
        let t27863 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1976(t111, t7982);
        let t27867 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1977(t1442, t1774, t2114, t25975, t25977, t25979, t25982, t25987, t25991, t25993, t25996, t25998, t26002, t26005, t27863, t5107, t672, t7264, t7408);
        let t27878 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1978(t1266, t2165, t2167, t2314, t26006, t26141, t26144, t26145, t26147, t26150, t26153, t26157, t4026, t4028, t4034, t5361, t7271, t7983, t7989);
        let (t27879, t27888, t27903) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1979(t1458, t7408, t2113, t671, t24932, t26109, t26111, t26113, t26116, t26119, t26121, t26123, t26125, t26137, t27371, t27863, t4072, t7266);
    (t27834, t27838, t27843, t27850, t27858, t27860, t27863, t27867, t27878, t27879, t27888, t27903)
}
