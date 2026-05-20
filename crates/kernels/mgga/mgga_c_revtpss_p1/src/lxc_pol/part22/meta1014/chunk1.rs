//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3492/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3492<F: Float>(t19716: F, t999: F, t11150: F, t11703: F, t11875: F, t13396: F, t15758: F, t15936: F, t15968: F, t15973: F, t16095: F, t1651: F, t19620: F, t19726: F, t19829: F, t20094: F, t20099: F, t20101: F, t3092: F, t3117: F, t42254: F, t43291: F, t4892: F, t4893: F, t4899: F, t53437: F, t53479: F, t54089: F, t6096: F, t6100: F) -> (F, F) {
    let t65773 = t19716 * t999;
    let t65795 = -F::cast_from(0.1270341277572436651e-3_f64) * t53437 + F::cast_from(0.28582678745379824648e-2_f64) * t16095 * t11703 * t1651 * t11150 * t15936 - F::cast_from(0.95275595817932748826e-3_f64) * t54089 * t20101 + F::cast_from(0.11433071498151929859e-2_f64) * t16095 * t3092 * t20094 * t13396 - F::cast_from(0.95275595817932748826e-3_f64) * t16095 * t11703 * t20099 * t13396 - F::cast_from(0.30488190661738479624e-2_f64) * t53479 + F::cast_from(0.57165357490759649296e-3_f64) * t15758 * t19726 + F::cast_from(0.85748036236139473944e-3_f64) * t11875 * t3117 * t4893 * t65773 + F::cast_from(0.28582678745379824648e-3_f64) * t4899 * t3092 * t6096 * t15973 + F::cast_from(0.28582678745379824648e-3_f64) * t4892 * t3092 * t6100 * t15968 - F::cast_from(0.14291339372689912324e-3_f64) * t4899 * t3092 * t6100 * t15973 - F::cast_from(0.25724410870841842184e-2_f64) * t43291 * t3117 * t19829 * t19620 - t42254 / F::new(1296.0);
    (t65773, t65795)
}
