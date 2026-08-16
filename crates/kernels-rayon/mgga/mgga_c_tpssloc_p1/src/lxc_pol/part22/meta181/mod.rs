//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta181 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1077;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1078;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1079;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1080;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1081;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1082;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1083;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1084;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta181(t1831: f64, t3866: f64, t1307: f64, t1799: f64, t3870: f64, t820: f64, t1367: f64, t5187: f64, t1341: f64, t1363: f64, t3781: f64, t3783: f64, t3800: f64, t3803: f64, t3864: f64, t3867: f64, t5259: f64, t5289: f64, t5293: f64, t5303: f64, t5257: f64, t539: f64, t1835: f64, t225: f64, t1385: f64, t1842: f64, t3887: f64, t3787: f64, t68: f64, t544: f64, t1824: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5306, t5308) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1077(t1831, t3866, t1307, t1799);
        let t5310 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1078(t3870, t5308, t820);
        let t5314 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1079(t1367, t5187, t820);
        let t5317 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1080(t1341, t1363, t1831, t3781, t3783, t3800, t3803, t3864, t3867, t5259, t5289, t5293, t5303, t5306, t5310, t5314);
        let t5318 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1081(t5257, t5317);
        let (t5319, t5321) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1082(t5318, t539, t1835, t225);
        let t5326 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1083(t1385, t1842, t3887);
        let (t5333, t5334) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1084(t3787, t68, t544);
        let t5335 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1085(t1824, t562);
    (t5306, t5308, t5310, t5314, t5318, t5319, t5321, t5326, t5333, t5334, t5335)
}
