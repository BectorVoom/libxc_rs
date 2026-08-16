//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta694 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2156;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2157;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2158;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2159;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta694(t28164: f64, t6914: f64, t22704: f64, t22705: f64, t28181: f64, t19889: f64, t91004: f64, t91006: f64, t28182: f64, t19660: f64, t22633: f64, t3807: f64, t6976: f64, t1336: f64, t22873: f64, t28171: f64, t28174: f64, t3777: f64, t5230: f64, t6420: f64, t7747: f64, t91002: f64, t91011: f64, t93605: f64, t93615: f64, t97119: f64, t97124: f64, t97129: f64, t97135: f64, t22685: f64, t22881: f64, t6330: f64, t6637: f64, t22893: f64, t28142: f64, t80681: f64, t2006: f64, t6387: f64, t28143: f64, t80727: f64, t6414: f64, t1824: f64, t7722: f64, t1338: f64, t28107: f64, t1352: f64, t16047: f64, t1814: f64, t1825: f64, t19654: f64, t19744: f64, t26401: f64, t26403: f64, t26453: f64, t5250: f64, t5287: f64, t5334: f64, t5344: f64, t81147: f64, t81149: f64, t81154: f64, t81187: f64, t81197: f64, t90952: f64) -> (f64, f64, f64, f64, f64) {
        let (t97137, t97142, t97146, t97148, t97152) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2156(t28164, t6914, t22704, t22705, t28181, t19889, t91004, t91006, t28182, t19660, t22633, t3807, t6976);
        let t97154 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2157(t1336, t22873, t28171, t28174, t3777, t5230, t6420, t7747, t91002, t91011, t93605, t93615, t97119, t97124, t97129, t97135, t97137, t97142, t97146, t97148, t97152);
        let (t97158, t97161, t97172, t97179, t97181) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2158(t22685, t22881, t6330, t6637, t22893, t28142, t80681, t2006, t6387, t28143, t80727, t6414);
        let (t97189, t97196) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2159(t1824, t7722, t1338, t28107, t1336, t1352, t16047, t1814, t1825, t19654, t19744, t26401, t26403, t26453, t5250, t5287, t5334, t5344, t81147, t81149, t81154, t81187, t81197, t90952, t97158, t97161, t97172, t97179, t97181);
    (t97154, t97172, t97181, t97189, t97196)
}
