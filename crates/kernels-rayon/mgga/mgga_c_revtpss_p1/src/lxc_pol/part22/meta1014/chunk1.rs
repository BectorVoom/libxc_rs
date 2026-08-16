//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3492/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3492(t19716: f64, t999: f64, t11150: f64, t11703: f64, t11875: f64, t13396: f64, t15758: f64, t15936: f64, t15968: f64, t15973: f64, t16095: f64, t1651: f64, t19620: f64, t19726: f64, t19829: f64, t20094: f64, t20099: f64, t20101: f64, t3092: f64, t3117: f64, t42254: f64, t43291: f64, t4892: f64, t4893: f64, t4899: f64, t53437: f64, t53479: f64, t54089: f64, t6096: f64, t6100: f64) -> (f64, f64) {
    let t65773 = t19716 * t999;
    let t65795 = -0.1270341277572436651e-3_f64 * t53437 + 0.28582678745379824648e-2_f64 * t16095 * t11703 * t1651 * t11150 * t15936 - 0.95275595817932748826e-3_f64 * t54089 * t20101 + 0.11433071498151929859e-2_f64 * t16095 * t3092 * t20094 * t13396 - 0.95275595817932748826e-3_f64 * t16095 * t11703 * t20099 * t13396 - 0.30488190661738479624e-2_f64 * t53479 + 0.57165357490759649296e-3_f64 * t15758 * t19726 + 0.85748036236139473944e-3_f64 * t11875 * t3117 * t4893 * t65773 + 0.28582678745379824648e-3_f64 * t4899 * t3092 * t6096 * t15973 + 0.28582678745379824648e-3_f64 * t4892 * t3092 * t6100 * t15968 - 0.14291339372689912324e-3_f64 * t4899 * t3092 * t6100 * t15973 - 0.25724410870841842184e-2_f64 * t43291 * t3117 * t19829 * t19620 - t42254 / 1296.0_f64;
    (t65773, t65795)
}
