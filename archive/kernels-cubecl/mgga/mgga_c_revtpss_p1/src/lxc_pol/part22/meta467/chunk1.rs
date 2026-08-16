//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2150/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2150<F: Float>(t4757: F, t4786: F, t3117: F, t11264: F, t11271: F, t11774: F, t11859: F, t11875: F, t11927: F, t15583: F, t15586: F, t15592: F, t15596: F, t15601: F, t15606: F, t15611: F, t15615: F, t15618: F, t3091: F, t3097: F) -> (F, F, F) {
    let t15621 = t4757 * t4786;
    let t15622 = t3117 * t15621;
    let t15625 = -t15583 - F::cast_from(0.28582678745379824648e-3_f64) * t11774 * t15586 - F::cast_from(0.95275595817932748826e-4_f64) * t11264 - F::cast_from(0.15244095330869239812e-2_f64) * t11271 + F::cast_from(0.14291339372689912324e-3_f64) * t3091 * t15592 + F::cast_from(0.23818898954483187207e-3_f64) * t3091 * t15596 + F::cast_from(0.14291339372689912324e-3_f64) * t3091 * t15601 + F::cast_from(0.42874018118069736972e-3_f64) * t11875 * t15606 - F::cast_from(0.85748036236139473944e-3_f64) * t11859 * t15611 + F::cast_from(0.28582678745379824648e-3_f64) * t3091 * t15615 + F::cast_from(0.28582678745379824648e-3_f64) * t15618 * t3097 + F::cast_from(0.85748036236139473944e-3_f64) * t11927 * t15622;
    (t15621, t15622, t15625)
}
