//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2816/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2816(t231: f64, t2782: f64, t2783: f64, t76169: f64, t62615: f64, t62619: f64, t62626: f64, t62630: f64, t62633: f64, t62635: f64, t62639: f64, t76153: f64, t76158: f64, t76163: f64) -> f64 {
    let t76172 = t2782 * t2783 * t76169 * t231;
    let t76174 = 0.16463622957338778996e-1_f64 * t62615 + 0.29272321618148349057e-1_f64 * t62619 - 0.58544643236296698112e-1_f64 * t76153 + 0.32927245914677557992e-1_f64 * t62626 + 0.58544643236296698112e-1_f64 * t76158 + 0.16463622957338778997e-1_f64 * t76163 - 0.65854491829355115984e-1_f64 * t62630 + 0.39029762157531132076e-1_f64 * t62633 - 0.29272321618148349057e-1_f64 * t62635 + 0.16463622957338778996e-1_f64 * t62639 + 0.54878743191129263322e-2_f64 * t76172;
    t76174
}
