//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1164/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1164(t35951: f64, t2001: f64, t5232: f64, t2327: f64, t7610: f64, t31676: f64, t31680: f64, t31682: f64, t31684: f64, t31687: f64, t35924: f64, t35927: f64, t35928: f64, t35931: f64, t35935: f64, t35936: f64, t35938: f64, t35942: f64, t35947: f64, t35949: f64) -> f64 {
    let t35952 = 0.17149607247227894789e-2_f64 * t35951;
    let t35953 = t2001 * t5232;
    let t35955 = t7610 * t2327;
    let t35957 = 13.0_f64 / 288.0_f64 * t35924 + t35927 - 0.34299214494455789578e-2_f64 * t35928 - t35931 - t35935 - 0.19865625e0_f64 * t35936 - 0.1324375e0_f64 * t35938 - 0.34299214494455789578e-2_f64 * t31676 + 0.85748036236139473944e-3_f64 * t31680 - 0.17149607247227894789e-2_f64 * t35942 + 0.55907719625962937008e-2_f64 * t31682 - 0.62896184579208304136e-3_f64 * t31684 - 0.14291339372689912324e-3_f64 * t31687 + 0.85748036236139473944e-3_f64 * t35947 - 0.85748036236139473944e-3_f64 * t35949 - t35952 - 0.85748036236139473944e-3_f64 * t35953 + 0.10718504529517434243e-3_f64 * t35955;
    t35957
}
