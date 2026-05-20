//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3007/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3007<F: Float>(t20020: F, t4858: F, t1011: F, t140: F, t23877: F, t1043: F, t24031: F, t15823: F, t20029: F, t1045: F, t15696: F, t15700: F, t19625: F, t19981: F, t23878: F, t24024: F, t3117: F, t3181: F, t3211: F, t3241: F, t372: F, t42328: F, t43069: F, t43291: F, t4782: F, t55034: F, t6299: F, t66306: F, t67152: F, t67186: F, t67195: F, t67199: F, t67206: F, t67213: F, t67237: F, t67249: F, t67253: F) -> (F, F) {
    let t79874 = t4858 * t20020;
    let t79881 = t1011 * t140 * t23877;
    let t79884 = t24031 * t1043;
    let t79892 = t15823 * t20029;
    let t79907 = F::cast_from(0.42874018118069736972e-3_f64) * t42328 * t15696 * t19625 - F::cast_from(0.42874018118069736972e-3_f64) * t79874 + F::cast_from(0.11433071498151929859e-2_f64) * t3211 * t24024 + F::new(2.0) / F::new(27.0) * t3241 * t23878 - t79881 / F::new(108.0) + F::cast_from(0.42874018118069736972e-3_f64) * t67152 - F::cast_from(0.12862205435420921092e-2_f64) * t43291 * t3117 * t79884 * t1045 + F::cast_from(0.14291339372689912324e-3_f64) * t67186 + F::cast_from(0.28582678745379824648e-3_f64) * t67195 + F::cast_from(0.85748036236139473944e-3_f64) * t67199 + F::cast_from(0.85748036236139473947e-3_f64) * t79892 - F::cast_from(0.28582678745379824648e-3_f64) * t67206 - F::cast_from(0.85748036236139473944e-3_f64) * t67213 + F::cast_from(0.85748036236139473944e-3_f64) * t67237 + F::cast_from(0.7145669686344956162e-3_f64) * t15700 * t372 * t3181 * t6299 * t19981 + F::cast_from(0.85748036236139473947e-3_f64) * t43069 * t66306 * t4782 + F::cast_from(0.28582678745379824648e-3_f64) * t67249 + t55034 + F::cast_from(0.17149607247227894789e-2_f64) * t67253;
    (t79884, t79907)
}
