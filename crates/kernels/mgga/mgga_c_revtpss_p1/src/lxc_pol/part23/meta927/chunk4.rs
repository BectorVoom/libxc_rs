//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3013/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3013<F: Float>(t11710: F, t23903: F, t4899: F, t11250: F, t11933: F, t15689: F, t15691: F, t15700: F, t16049: F, t16222: F, t16226: F, t19622: F, t19626: F, t19741: F, t19996: F, t23904: F, t23939: F, t23999: F, t3091: F, t3092: F, t3095: F, t3162: F, t43066: F, t4579: F, t54500: F, t54658: F, t54672: F, t54801: F, t55294: F, t66187: F, t67560: F, t67568: F, t67571: F, t67575: F, t79159: F, t79395: F, t79463: F, t79467: F, t79770: F) -> F {
    let t80113 = t4899 * t11710 * t23903;
    let t80127 = t55294 - F::cast_from(0.25724410870841842183e-2_f64) * t54801 * t66187 * t11250 * t19996 + F::cast_from(0.45732285992607719437e-2_f64) * t43066 * t23939 - F::cast_from(0.42874018118069736972e-2_f64) * t15700 * t54658 * t79395 + F::cast_from(0.14291339372689912324e-2_f64) * t15700 * t16222 * t79463 + F::cast_from(0.14291339372689912324e-2_f64) * t16226 * t16222 * t79467 + F::cast_from(0.19055119163586549765e-2_f64) * t15700 * t54672 * t79395 - F::cast_from(0.85748036236139473944e-3_f64) * t15689 * t15691 * t3162 * t79770 + F::cast_from(0.85748036236139473944e-3_f64) * t15689 * t66187 * t3162 * t4579 + F::cast_from(0.34299214494455789578e-2_f64) * t11933 * t23999 + F::cast_from(0.22866142996303859718e-2_f64) * t16049 * t23904 - F::cast_from(0.28582678745379824648e-3_f64) * t80113 - F::cast_from(0.42874018118069736972e-3_f64) * t19741 * t19626 + F::cast_from(0.14291339372689912324e-3_f64) * t3091 * t3092 * t79159 * t3095 + F::cast_from(0.25724410870841842184e-2_f64) * t54500 * t19622 + F::cast_from(0.17149607247227894789e-2_f64) * t67560 + F::cast_from(0.42874018118069736972e-3_f64) * t67568 + F::cast_from(0.30488190661738479624e-2_f64) * t67571 + F::cast_from(0.19055119163586549765e-3_f64) * t67575;
    t80127
}
