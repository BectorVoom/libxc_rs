//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1900;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1901;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1902;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1903;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta549<F: Float>(t491: F, t8034: F, t7287: F, t24567: F, t8014: F, t225: F, t8018: F, t1252: F, t15797: F, t2155: F, t24589: F, t24891: F, t27800: F, t27805: F, t27808: F, t27812: F, t27818: F, t3487: F, t4945: F, t498: F, t5055: F, t5089: F, t7283: F, t7296: F, t7351: F, t7356: F, t7392: F, t7999: F, t8088: F, t265: F, t504: F, t27421: F, t27757: F, t27797: F, t3640: F, t8090: F, t1254: F, t1763: F, t1256: F, t193: F, t24905: F, t24909: F, t25882: F, t336: F, t4700: F, t5091: F, t7398: F, t28: F, t1409: F, t2161: F, t25949: F, t3966: F, t52: F, t607: F, t7402: F, t8097: F, t27380: F, t113: F, t24988: F, t24989: F, t24993: F, t24998: F, t25005: F, t25007: F, t25011: F, t25969: F, t25973: F, t27290: F, t27293: F, t27371: F, t510: F, t650: F, t652: F, t8103: F, dens_threshold: F, rho1: F, zeta_threshold: F, t111: F, t7982: F, t1442: F, t1774: F, t2114: F, t25975: F, t25977: F, t25979: F, t25982: F, t25987: F, t25991: F, t25993: F, t25996: F, t25998: F, t26002: F, t26005: F, t5107: F, t672: F, t7264: F, t7408: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27820, t27821, t27826, t27830, t27832) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1900::<F>(t491, t8034, t7287, t24567, t8014, t225, t8018, t1252, t15797, t2155, t24589, t24891, t27800, t27805, t27808, t27812, t27818, t3487, t4945, t498, t5055, t5089, t7283, t7296, t7351, t7356, t7392, t7999, t8088);
        let (t27834, t27838, t27843, t27850) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1901::<F>(t265, t504, t27421, t27757, t27797, t27832, t3640, t8090, t1254, t1763, t1256, t193, t24905, t24909, t25882, t336, t4700, t5091, t7398);
        let (t27858, t27860) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1902::<F>(t28, t1409, t2161, t25949, t27850, t3966, t52, t607, t7402, t8097, t27380, t113, t24988, t24989, t24993, t24998, t25005, t25007, t25011, t25969, t25973, t27290, t27293, t27371, t510, t650, t652, t8103, dens_threshold, rho1, zeta_threshold);
        let t27863 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1903::<F>(t111, t7982);
        let t27867 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1904::<F>(t1442, t1774, t2114, t25975, t25977, t25979, t25982, t25987, t25991, t25993, t25996, t25998, t26002, t26005, t27863, t5107, t672, t7264, t7408);
    (t27820, t27821, t27826, t27830, t27834, t27838, t27843, t27850, t27858, t27860, t27863, t27867)
}
