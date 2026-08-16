//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta642 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2113;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2114;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2115;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2116;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta642(t87233: f64, t25068: f64, t2703: f64, t81764: f64, t23127: f64, t4257: f64, t1512: f64, t81807: f64, t25146: f64, t2686: f64, t81824: f64, t81821: f64, t23053: f64, t4236: f64, t13173: f64, t6614: f64, t23041: f64, t13186: f64, t6621: f64, t81770: f64, t81772: f64, t81785: f64, t87222: f64, t87224: f64, t87226: f64, t23040: f64, t4166: f64, t831: f64, t81808: f64, t4191: f64, t81749: f64, t4240: f64, t13248: f64, t25084: f64, t13326: f64, t23146: f64, t13210: f64, t13306: f64, t13231: f64, t81789: f64, t81795: f64, t81797: f64, t81799: f64, t81810: f64, t81825: f64, t81836: f64, t81850: f64, t81853: f64, t13353: f64, t13225: f64, t23069: f64, t4159: f64, t23062: f64, t25106: f64, t13176: f64, t6613: f64, t2681: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87234, t87235, t87237, t87241, t87243, t87245, t87248, t87249) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2113(t87233, t25068, t2703, t81764, t23127, t4257, t1512, t81807, t25146, t2686, t81824, t81821);
        let t87259 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2114(t23053, t4236, t13173, t6614, t23041, t13186, t6621, t81770, t81772, t81785, t87222, t87224, t87226, t87234, t87235, t87237, t87241, t87243, t87245, t87248, t87249);
        let (t87263, t87268, t87271, t87273, t87274, t87276, t87278) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2115(t23040, t4166, t831, t81808, t4191, t81749, t4240, t13248, t25084, t13326, t23146, t13210);
        let t87286 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2116(t13306, t23146, t13231, t25084, t81789, t81795, t81797, t81799, t81810, t81825, t81836, t81850, t81853, t87263, t87268, t87271, t87273, t87274, t87276, t87278);
        let (t87287, t87289, t87292, t87293, t87296, t87298) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2117(t13353, t23146, t13225, t23069, t4159, t23062, t25106, t13176, t6613, t831, t25146, t2681);
    (t87259, t87286, t87287, t87289, t87292, t87293, t87296, t87298)
}
