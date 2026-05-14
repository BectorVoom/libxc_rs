//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1166/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1166<F: Float>(t3375: F, t6405: F, t3431: F, t5618: F, t435: F, t6068: F, t6116: F, t997: F, t145: F, t5784: F, t3382: F, t5801: F, t1165: F, t1173: F, t1180: F, t1181: F, t1188: F, t18743: F, t18747: F, t18763: F, t18765: F, t21342: F, t301: F, t335: F, t336: F, t4437: F, t4680: F, t530: F, t5867: F, t6395: F) -> (F,) {
    let t24173 = t3375 * t6405;
    let t24175 = t3431 * t5618;
    let t24184 = t435 * t6068;
    let t24194 = t997 * t6116;
    let t24196 = t5784 * t145;
    let t24201 = t3382 * t5801;
    let t24204 = -0.10289764348336736873e-1 * t18743 - 0.68598428988911579156e-2 * t18747 - 0.17149607247227894789e-2 * t24173 - 0.32012600194825403606e-1 * t24175 + 0.34299214494455789578e-2 * t1173 * t1181 * t530 * t21342 + 0.34299214494455789578e-2 * t1180 * t4680 * t6395 + 0.85748036236139473944e-3 * t1180 * t1165 * t24184 * t1188 + 0.42874018118069736972e-3 * t1180 * t1165 * t5867 * t4437 - 0.34299214494455789578e-2 * t18763 - 0.40015750243531754508e-2 * t24194 - t335 * t336 * t24196 * t301 / 24.0 - 0.17149607247227894789e-2 * t24201 - 0.42874018118069736972e-3 * t18765;
    (t24204,)
}
