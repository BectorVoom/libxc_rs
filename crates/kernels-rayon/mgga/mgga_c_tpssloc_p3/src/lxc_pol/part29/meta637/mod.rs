//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2091;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2092;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2093;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2094;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2095;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2096;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta637(t22986: f64, t25054: f64, t82159: f64, t23168: f64, t25229: f64, t23222: f64, t25224: f64, t6552: f64, t1519: f64, t794: f64, t23164: f64, t6555: f64, t23035: f64, t23241: f64, t7480: f64, t81632: f64, t22975: f64, t23191: f64, t25184: f64, t25330: f64, t2597: f64, t2713: f64, t4147: f64, t4268: f64, t86844: f64, t86847: f64, t86852: f64, t86857: f64, t86862: f64, t86866: f64, t86869: f64, t86870: f64, t86875: f64, t86881: f64, t25038: f64, t25040: f64, t23030: f64, t25035: f64, t23228: f64, t7479: f64, t81573: f64, t23270: f64, t25191: f64, t2742: f64, t25059: f64, t6562: f64, t82082: f64, t82087: f64, t1888: f64, t25045: f64, t7488: f64, t82133: f64, t25225: f64, t6547: f64, t25338: f64, t13059: f64, t22979: f64, t2718: f64, t6627: f64, t7537: f64, t855: f64, t23012: f64, t7485: f64, t2719: f64, t46488: f64, t25046: f64, t6579: f64, t1484: f64, t2717: f64, t82099: f64, t225: f64, t25051: f64, t7489: f64, t82120: f64, t13460: f64, t1880: f64, t6553: f64, t6571: f64, t1527: f64, t23190: f64, t25160: f64, t259: f64, t2591: f64, t7510: f64, t798: f64, t82108: f64, t82115: f64, t82123: f64, t866: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86884, t86887, t86891, t86893, t86895) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2091(t22986, t25054, t82159, t23168, t25229, t23222, t25224, t6552, t1519, t794, t23164, t6555);
        let t86905 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2092(t86895, t23035, t23241, t25224, t7480, t81632, t22975, t23191, t25184, t25330, t2597, t2713, t4147, t4268, t86844, t86847, t86852, t86857, t86862, t86866, t86869, t86870, t86875, t86881, t86884, t86887, t86891);
        let (t86909, t86911, t86916, t86923) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2093(t25038, t25040, t82159, t23030, t25035, t23228, t7479, t81573, t22986, t23270, t25191, t2742);
        let (t86929, t86930, t86931, t86933, t86941, t86942) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2094(t25059, t6562, t794, t82082, t82087, t1888, t25045, t82159, t7488, t82133, t25225, t6547);
        let t86952 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2095(t86942, t23168, t25338, t13059, t22979, t25184, t2713, t2718, t2742, t4268, t6627, t7537, t855, t86929, t86930, t86931, t86933, t86941);
        let (t86955, t86961, t86968, t86972, t86983) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2096(t23012, t7485, t1888, t23270, t2719, t46488, t25046, t6579, t1484, t2717, t22986, t82099);
        let t87005 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2097(t225, t25051, t23012, t7489, t82120, t13460, t1880, t6553, t6571, t1527, t23190, t25160, t259, t2591, t2718, t7510, t798, t82108, t82115, t82123, t855, t866, t86983);
    (t86893, t86905, t86909, t86911, t86916, t86923, t86952, t86955, t86961, t86968, t86972, t87005)
}
