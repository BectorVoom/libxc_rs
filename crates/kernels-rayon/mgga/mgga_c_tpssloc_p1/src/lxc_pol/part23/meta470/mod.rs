//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1398;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1399;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1400;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1401;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta470(t77119: f64, t77122: f64, t77124: f64, t77127: f64, t77130: f64, t77133: f64, t77135: f64, t77138: f64, t77143: f64, t77145: f64, t77148: f64, t77150: f64, t77153: f64, t5946: f64, t193: f64, t3216: f64, t336: f64, t4700: f64, t5950: f64, t60874: f64, t77157: f64, t77159: f64, t77224: f64, t77226: f64, t77229: f64, t77232: f64, t77236: f64, t77470: f64, t77474: f64, t77478: f64, t77482: f64, t25: f64, t265: f64, t394: f64, t76559: f64, t76666: f64, t77918: f64, t1408: f64, t1409: f64, t1534: f64, t1642: f64, t20216: f64, t20217: f64, t21076: f64, t21703: f64, t396: f64, t40: f64, t5397: f64, t5398: f64, t5669: f64, t5955: f64, t75911: f64, t75912: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t43791: f64, t75836: f64, t11219: f64, t136: f64, t43763: f64, t43761: f64, t3242: f64, t75847: f64, t3297: f64, t3247: f64, t1113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t77920 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1398(t77119, t77122, t77124, t77127, t77130, t77133, t77135, t77138, t77143, t77145, t77148, t77150, t77153);
        let t77929 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1399(t5946, t193, t3216, t336, t4700, t5950, t60874, t77157, t77159, t77224, t77226, t77229, t77232, t77236, t77470, t77474, t77478, t77482);
        let t77944 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1400(t25, t265, t394, t76559, t76666, t77918, t77920, t77929, t1408, t1409, t1534, t1642, t20216, t20217, t21076, t21703, t396, t40, t5397, t5398, t5669, t5955, t75911, t75912, dens_threshold, rho0, zeta_threshold);
        let (t77953, t77957, t77959, t77961, t77963, t77965, t77967, t77969, t77971) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1401(t75911, t43791, t75836, t11219, t136, t43763, t43761, t3242, t75847, t3297, t3247, t1113);
    (t77944, t77953, t77957, t77959, t77961, t77963, t77965, t77967, t77969, t77971)
}
