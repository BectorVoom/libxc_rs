//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1673;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1674;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1675;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta490(t225: f64, t7910: f64, t26231: f64, t26251: f64, t26255: f64, t26266: f64, t22785: f64, t22795: f64, t26258: f64, t26260: f64, t26262: f64, t26268: f64, t26272: f64, t26274: f64, t26278: f64, t22856: f64, t22861: f64, t24058: f64, t24060: f64, t24061: f64, t26306: f64, t26310: f64, t26312: f64, t26314: f64, t26320: f64, t26324: f64, t22767: f64, t22780: f64, t22799: f64, t22805: f64, t24049: f64, t24050: f64, t26234: f64, t26236: f64, t26238: f64, t26240: f64, t26246: f64, t26249: f64, t26286: f64, t26290: f64, t26293: f64, t26295: f64, t26299: f64, t26303: f64, t539: f64, t1323: f64, t7918: f64, t1385: f64, t7936: f64, t3887: f64, t1375: f64, t1386: f64, t16030: f64, t2092: f64, t24071: f64, t26217: f64, t26335: f64, t26340: f64, t26345: f64, t26352: f64, t26357: f64, t3882: f64, t568: f64, t7925: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t27009, t27012, t27019, t27032) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1673(t225, t7910, t26231, t26251, t26255, t26266, t22785, t22795, t26258, t26260, t26262, t26268, t26272, t26274, t26278);
        let t27051 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1674(t22856, t22861, t24058, t24060, t24061, t26306, t26310, t26312, t26314, t26320, t26324, t22767, t22780, t22799, t22805, t24049, t24050, t26234, t26236, t26238, t26240, t26246, t26249, t26286, t26290, t26293, t26295, t26299, t26303, t27012, t27019, t27032);
        let (t27052, t27059, t27062, t27065) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1675(t27051, t539, t1323, t7918, t1385, t7936, t3887, t1375, t1386, t16030, t2092, t24071, t26217, t26335, t26340, t26345, t26352, t26357, t27009, t3882, t568, t7925);
    (t27009, t27051, t27052, t27059, t27062, t27065)
}
