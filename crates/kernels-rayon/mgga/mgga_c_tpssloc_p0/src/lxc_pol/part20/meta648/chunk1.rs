//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2380/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2380(t10813: f64, t4433: f64, t10743: f64, t10765: f64, t10771: f64, t10805: f64, t10811: f64, t14266: f64, t14328: f64, t14432: f64, t14435: f64, t14436: f64, t14442: f64, t1569: f64, t2861: f64, t2862: f64, t2880: f64, t2881: f64, t2886: f64, t2888: f64, t2889: f64, t311: f64, t41984: f64, t42154: f64, t47791: f64, t48747: f64, t48750: f64, t48765: f64, t48789: f64, t48813: f64, t48833: f64, t931: f64) -> f64 {
    let t48854 = t4433 * t10813;
    let t48861 = t48747 + t48750 + 3.0_f64 * t14266 * t2881 + 0.96491876992155210402e2_f64 * t48789 * t2889 - 0.19751673498613801407e-1_f64 * t47791 - 0.310907e-1_f64 * (t48813 + t48833) * t311 + t48765 - 2.0_f64 * t2861 * t1569 * t10805 - 0.57895126195293126242e3_f64 * t41984 * t14432 - 0.24828486201251232145e5_f64 * t42154 * t14442 * t10743 + 0.19298375398431042081e3_f64 * t10765 * t14436 + 0.96491876992155210402e2_f64 * t2886 * t14328 * t2888 * t931 + 0.96491876992155210402e2_f64 * t2886 * t14435 * t2880 + 0.6207121550312808036e4_f64 * t10811 * t48854 * t2862 - 24.0_f64 * t10771 * t1569 * t10743;
    t48861
}
