//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3537/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3537<F: Float>(t11875: F, t11922: F, t19757: F, t1045: F, t11144: F, t11774: F, t11866: F, t15586: F, t15595: F, t15689: F, t15701: F, t15936: F, t16049: F, t16095: F, t1651: F, t19626: F, t19864: F, t20038: F, t20091: F, t3115: F, t3117: F, t42155: F, t42328: F, t42410: F, t4574: F, t4900: F, t54994: F, t55000: F, t55122: F, t55141: F, t55209: F, t65060: F, t66734: F, t67090: F) -> F {
    let t67152 = t11875 * t11922 * t19757;
    let t67182 = -F::cast_from(0.1270341277572436651e-2_f64) * t16095 * t42410 * t1651 * t11144 * t15936 + F::cast_from(0.28582678745379824648e-3_f64) * t67152 - F::cast_from(0.57165357490759649296e-3_f64) * t54994 - F::cast_from(0.3811023832717309953e-3_f64) * t55000 - F::cast_from(0.57165357490759649296e-3_f64) * t55141 * t15586 - F::cast_from(0.47637797908966374413e-3_f64) * t11774 * t55122 * t15595 - F::cast_from(0.57165357490759649296e-3_f64) * t42155 * t19864 + F::cast_from(0.57165357490759649296e-3_f64) * t11774 * t15701 * t67090 - F::cast_from(0.95275595817932748826e-3_f64) * t15689 * t66734 * t4900 * t4574 + F::cast_from(0.57165357490759649296e-3_f64) * t42328 * t55209 * t4900 * t20038 + F::cast_from(0.15244095330869239812e-2_f64) * t16049 * t19626 - F::cast_from(0.85748036236139473944e-3_f64) * t11866 * t20091 - F::cast_from(0.42874018118069736972e-3_f64) * t3115 * t3117 * t65060 * t1045;
    t67182
}
