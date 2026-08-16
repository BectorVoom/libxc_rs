//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1900;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1901;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1902;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1903;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta549(t491: f64, t8034: f64, t7287: f64, t24567: f64, t8014: f64, t225: f64, t8018: f64, t1252: f64, t15797: f64, t2155: f64, t24589: f64, t24891: f64, t27800: f64, t27805: f64, t27808: f64, t27812: f64, t27818: f64, t3487: f64, t4945: f64, t498: f64, t5055: f64, t5089: f64, t7283: f64, t7296: f64, t7351: f64, t7356: f64, t7392: f64, t7999: f64, t8088: f64, t265: f64, t504: f64, t27421: f64, t27757: f64, t27797: f64, t3640: f64, t8090: f64, t1254: f64, t1763: f64, t1256: f64, t193: f64, t24905: f64, t24909: f64, t25882: f64, t336: f64, t4700: f64, t5091: f64, t7398: f64, t28: f64, t1409: f64, t2161: f64, t25949: f64, t3966: f64, t52: f64, t607: f64, t7402: f64, t8097: f64, t27380: f64, t113: f64, t24988: f64, t24989: f64, t24993: f64, t24998: f64, t25005: f64, t25007: f64, t25011: f64, t25969: f64, t25973: f64, t27290: f64, t27293: f64, t27371: f64, t510: f64, t650: f64, t652: f64, t8103: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t111: f64, t7982: f64, t1442: f64, t1774: f64, t2114: f64, t25975: f64, t25977: f64, t25979: f64, t25982: f64, t25987: f64, t25991: f64, t25993: f64, t25996: f64, t25998: f64, t26002: f64, t26005: f64, t5107: f64, t672: f64, t7264: f64, t7408: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27820, t27821, t27826, t27830, t27832) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1900(t491, t8034, t7287, t24567, t8014, t225, t8018, t1252, t15797, t2155, t24589, t24891, t27800, t27805, t27808, t27812, t27818, t3487, t4945, t498, t5055, t5089, t7283, t7296, t7351, t7356, t7392, t7999, t8088);
        let (t27834, t27838, t27843, t27850) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1901(t265, t504, t27421, t27757, t27797, t27832, t3640, t8090, t1254, t1763, t1256, t193, t24905, t24909, t25882, t336, t4700, t5091, t7398);
        let (t27858, t27860) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1902(t28, t1409, t2161, t25949, t27850, t3966, t52, t607, t7402, t8097, t27380, t113, t24988, t24989, t24993, t24998, t25005, t25007, t25011, t25969, t25973, t27290, t27293, t27371, t510, t650, t652, t8103, dens_threshold, rho1, zeta_threshold);
        let t27863 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1903(t111, t7982);
        let t27867 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1904(t1442, t1774, t2114, t25975, t25977, t25979, t25982, t25987, t25991, t25993, t25996, t25998, t26002, t26005, t27863, t5107, t672, t7264, t7408);
    (t27820, t27821, t27826, t27830, t27834, t27838, t27843, t27850, t27858, t27860, t27863, t27867)
}
