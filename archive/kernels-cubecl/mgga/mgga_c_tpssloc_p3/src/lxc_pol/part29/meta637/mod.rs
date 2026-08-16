//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2091;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2092;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2093;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2094;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2095;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2096;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta637<F: Float>(t22986: F, t25054: F, t82159: F, t23168: F, t25229: F, t23222: F, t25224: F, t6552: F, t1519: F, t794: F, t23164: F, t6555: F, t23035: F, t23241: F, t7480: F, t81632: F, t22975: F, t23191: F, t25184: F, t25330: F, t2597: F, t2713: F, t4147: F, t4268: F, t86844: F, t86847: F, t86852: F, t86857: F, t86862: F, t86866: F, t86869: F, t86870: F, t86875: F, t86881: F, t25038: F, t25040: F, t23030: F, t25035: F, t23228: F, t7479: F, t81573: F, t23270: F, t25191: F, t2742: F, t25059: F, t6562: F, t82082: F, t82087: F, t1888: F, t25045: F, t7488: F, t82133: F, t25225: F, t6547: F, t25338: F, t13059: F, t22979: F, t2718: F, t6627: F, t7537: F, t855: F, t23012: F, t7485: F, t2719: F, t46488: F, t25046: F, t6579: F, t1484: F, t2717: F, t82099: F, t225: F, t25051: F, t7489: F, t82120: F, t13460: F, t1880: F, t6553: F, t6571: F, t1527: F, t23190: F, t25160: F, t259: F, t2591: F, t7510: F, t798: F, t82108: F, t82115: F, t82123: F, t866: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t86884, t86887, t86891, t86893, t86895) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2091::<F>(t22986, t25054, t82159, t23168, t25229, t23222, t25224, t6552, t1519, t794, t23164, t6555);
        let t86905 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2092::<F>(t86895, t23035, t23241, t25224, t7480, t81632, t22975, t23191, t25184, t25330, t2597, t2713, t4147, t4268, t86844, t86847, t86852, t86857, t86862, t86866, t86869, t86870, t86875, t86881, t86884, t86887, t86891);
        let (t86909, t86911, t86916, t86923) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2093::<F>(t25038, t25040, t82159, t23030, t25035, t23228, t7479, t81573, t22986, t23270, t25191, t2742);
        let (t86929, t86930, t86931, t86933, t86941, t86942) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2094::<F>(t25059, t6562, t794, t82082, t82087, t1888, t25045, t82159, t7488, t82133, t25225, t6547);
        let t86952 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2095::<F>(t86942, t23168, t25338, t13059, t22979, t25184, t2713, t2718, t2742, t4268, t6627, t7537, t855, t86929, t86930, t86931, t86933, t86941);
        let (t86955, t86961, t86968, t86972, t86983) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2096::<F>(t23012, t7485, t1888, t23270, t2719, t46488, t25046, t6579, t1484, t2717, t22986, t82099);
        let t87005 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2097::<F>(t225, t25051, t23012, t7489, t82120, t13460, t1880, t6553, t6571, t1527, t23190, t25160, t259, t2591, t2718, t7510, t798, t82108, t82115, t82123, t855, t866, t86983);
    (t86893, t86905, t86909, t86911, t86916, t86923, t86952, t86955, t86961, t86968, t86972, t87005)
}
