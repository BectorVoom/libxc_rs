//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2787/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2787(t51207: f64, t11015: f64, t4325: f64, t4477: f64, t9292: f64, t14472: f64, t2439: f64, t887: f64, t213: f64, t225: f64, t257: f64, t2770: f64, t2828: f64, t41038: f64, t41043: f64, t41049: f64, t41052: f64, t41056: f64, t41058: f64, t4533: f64, t51184: f64, t51196: f64, t51199: f64, t51203: f64, t865: f64) -> f64 {
    let t51208 = 0.69394917116090352834e-2_f64 * t51207;
    let t51211 = t4325 * t11015;
    let t51213 = t9292 * t4477;
    let t51216 = t2439 * t14472 * t887;
    let t51217 = 0.19514881078765566038e-2_f64 * t51216;
    let t51218 = 0.65854491829355115987e0_f64 * t213 * t51184 * t225 * t257 + 0.78059524315062264151e-1_f64 * t41038 + 0.39512695097613069591e1_f64 * t865 * t2770 * t4533 * t2828 + 0.58544643236296698114e-1_f64 * t41043 + 0.32927245914677557992e-1_f64 * t51196 + 0.19514881078765566037e-2_f64 * t51199 + t41049 - 0.78059524315062264151e-1_f64 * t41052 + 0.46263278077393568556e-2_f64 * t51203 + t51208 - 0.34697458558045176417e-2_f64 * t41056 - 0.29272321618148349057e-1_f64 * t41058 + 0.30356481678079769392e-1_f64 * t51211 + 0.17073386770573548589e-1_f64 * t51213 + t51217;
    t51218
}
