//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 763/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk763(t232: f64, t4233: f64, t819: f64, t820: f64, t4180: f64, t4181: f64, t829: f64, t120: f64, t1484: f64, t2645: f64, t1516: f64, t2697: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4234 = t4233 * t232;
    let t4236 = t819 * t820 * t4234;
    let t4240 = t4180 * t4181 * t829;
    let t4248 = t120 * t1484;
    let t4250 = t2645 * t4248 * t829;
    let t4253 = t2697 * t1516;
    (t4234, t4236, t4240, t4248, t4250, t4253)
}
