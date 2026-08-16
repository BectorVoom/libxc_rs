//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 821/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk821(t44512: f64, t13273: f64, t2312: f64, t42820: f64, t13258: f64, t2321: f64, t38051: f64, t9074: f64, t1063: f64, t3565: f64, t6750: f64, t2268: f64, t2765: f64, t34267: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44513 = 0.11856252764865062333e-2_f64 * t44512;
    let t44514 = t2312 * t13273;
    let t44515 = 0.23712505529730124666e-2_f64 * t44514;
    let t44516 = 0.142275033178380748e-1_f64 * t42820;
    let t44517 = t2312 * t13258;
    let t44518 = 0.11856252764865062333e-2_f64 * t44517;
    let t44520 = t9074 * t38051 * t2321;
    let t44521 = 0.11856252764865062333e-2_f64 * t44520;
    let t44524 = 0.19918504644973304719e0_f64 * t1063 * t3565 * t6750;
    let t44527 = 0.39837009289946609438e0_f64 * t2268 * t2765 * t34267;
    (t44513, t44515, t44516, t44518, t44521, t44524, t44527)
}
