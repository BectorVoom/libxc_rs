//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta218 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1327;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1328;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1329;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1330;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1331;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1332;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta218(t1367: f64, t5187: f64, t820: f64, t1341: f64, t1363: f64, t1831: f64, t3781: f64, t3783: f64, t3800: f64, t3803: f64, t3864: f64, t3867: f64, t5259: f64, t5289: f64, t5293: f64, t5303: f64, t5306: f64, t5310: f64, t5257: f64, t539: f64, t1835: f64, t225: f64, t1385: f64, t1842: f64, t3887: f64, t3787: f64, t68: f64, t544: f64, t1824: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5314 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1327(t1367, t5187, t820);
        let t5317 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1328(t1341, t1363, t1831, t3781, t3783, t3800, t3803, t3864, t3867, t5259, t5289, t5293, t5303, t5306, t5310, t5314);
        let t5318 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1329(t5257, t5317);
        let (t5319, t5321) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1330(t5318, t539, t1835, t225);
        let t5326 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1331(t1385, t1842, t3887);
        let (t5333, t5334) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1332(t3787, t68, t544);
        let t5335 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1333(t1824, t562);
    (t5314, t5318, t5319, t5321, t5326, t5333, t5334, t5335)
}
