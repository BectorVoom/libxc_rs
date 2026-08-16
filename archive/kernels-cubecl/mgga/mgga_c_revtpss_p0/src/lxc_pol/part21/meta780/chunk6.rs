//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2787/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2787<F: Float>(t51207: F, t11015: F, t4325: F, t4477: F, t9292: F, t14472: F, t2439: F, t887: F, t213: F, t225: F, t257: F, t2770: F, t2828: F, t41038: F, t41043: F, t41049: F, t41052: F, t41056: F, t41058: F, t4533: F, t51184: F, t51196: F, t51199: F, t51203: F, t865: F) -> F {
    let t51208 = F::cast_from(0.69394917116090352834e-2_f64) * t51207;
    let t51211 = t4325 * t11015;
    let t51213 = t9292 * t4477;
    let t51216 = t2439 * t14472 * t887;
    let t51217 = F::cast_from(0.19514881078765566038e-2_f64) * t51216;
    let t51218 = F::cast_from(0.65854491829355115987e0_f64) * t213 * t51184 * t225 * t257 + F::cast_from(0.78059524315062264151e-1_f64) * t41038 + F::cast_from(0.39512695097613069591e1_f64) * t865 * t2770 * t4533 * t2828 + F::cast_from(0.58544643236296698114e-1_f64) * t41043 + F::cast_from(0.32927245914677557992e-1_f64) * t51196 + F::cast_from(0.19514881078765566037e-2_f64) * t51199 + t41049 - F::cast_from(0.78059524315062264151e-1_f64) * t41052 + F::cast_from(0.46263278077393568556e-2_f64) * t51203 + t51208 - F::cast_from(0.34697458558045176417e-2_f64) * t41056 - F::cast_from(0.29272321618148349057e-1_f64) * t41058 + F::cast_from(0.30356481678079769392e-1_f64) * t51211 + F::cast_from(0.17073386770573548589e-1_f64) * t51213 + t51217;
    t51218
}
