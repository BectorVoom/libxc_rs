//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 964/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk964<F: Float>(t1125: F, t1305: F, t1127: F, t1312: F, t1129: F, t1131: F, t1133: F, t1135: F, t1137: F, t1310: F, t3526: F, t3530: F, t3534: F, t3538: F, t3542: F, t839: F) -> (F, F, F) {
    let t11244 = t1125 * t1305;
    let t11249 = t1312 * t1127;
    let t11273 = -F::cast_from(0.9214113627294e1_f64) * t11249 - F::cast_from(0.18428227254588e2_f64) * t3526 * t839 - F::cast_from(0.9214113627294e1_f64) * t1129 * t1310 + F::cast_from(0.734774460522e2_f64) * t3530 * t839 + F::cast_from(0.367387230261e2_f64) * t1131 * t1310 - F::cast_from(0.7662840944824e2_f64) * t3534 * t839 - F::cast_from(0.3831420472412e2_f64) * t1133 * t1310 + F::cast_from(0.3101306810232e2_f64) * t3538 * t839 + F::cast_from(0.1550653405116e2_f64) * t1135 * t1310 - F::cast_from(0.4355305902528e1_f64) * t3542 * t839 - F::cast_from(0.2177652951264e1_f64) * t1137 * t1310 + F::cast_from(0.734774460522e2_f64) * t1129 * t1312;
    (t11244, t11249, t11273)
}
