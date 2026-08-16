//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1673;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1674;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1675;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta490<F: Float>(t225: F, t7910: F, t26231: F, t26251: F, t26255: F, t26266: F, t22785: F, t22795: F, t26258: F, t26260: F, t26262: F, t26268: F, t26272: F, t26274: F, t26278: F, t22856: F, t22861: F, t24058: F, t24060: F, t24061: F, t26306: F, t26310: F, t26312: F, t26314: F, t26320: F, t26324: F, t22767: F, t22780: F, t22799: F, t22805: F, t24049: F, t24050: F, t26234: F, t26236: F, t26238: F, t26240: F, t26246: F, t26249: F, t26286: F, t26290: F, t26293: F, t26295: F, t26299: F, t26303: F, t539: F, t1323: F, t7918: F, t1385: F, t7936: F, t3887: F, t1375: F, t1386: F, t16030: F, t2092: F, t24071: F, t26217: F, t26335: F, t26340: F, t26345: F, t26352: F, t26357: F, t3882: F, t568: F, t7925: F) -> (F, F, F, F, F, F) {
        let (t27009, t27012, t27019, t27032) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1673::<F>(t225, t7910, t26231, t26251, t26255, t26266, t22785, t22795, t26258, t26260, t26262, t26268, t26272, t26274, t26278);
        let t27051 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1674::<F>(t22856, t22861, t24058, t24060, t24061, t26306, t26310, t26312, t26314, t26320, t26324, t22767, t22780, t22799, t22805, t24049, t24050, t26234, t26236, t26238, t26240, t26246, t26249, t26286, t26290, t26293, t26295, t26299, t26303, t27012, t27019, t27032);
        let (t27052, t27059, t27062, t27065) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1675::<F>(t27051, t539, t1323, t7918, t1385, t7936, t3887, t1375, t1386, t16030, t2092, t24071, t26217, t26335, t26340, t26345, t26352, t26357, t27009, t3882, t568, t7925);
    (t27009, t27051, t27052, t27059, t27062, t27065)
}
