//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 977/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk977(t2268: f64, t2765: f64, t9152: f64, t39791: f64, t39794: f64, t39798: f64, t12830: f64, t29874: f64, t39805: f64, t39808: f64, t39811: f64, t12803: f64, t1358: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42814 = 0.85365019907028448797e-1_f64 * t2268 * t2765 * t9152;
    let t42815 = 0.23712505529730124666e-2_f64 * t39791;
    let t42816 = 0.23712505529730124666e-2_f64 * t39794;
    let t42817 = 0.23712505529730124666e-2_f64 * t39798;
    let t42820 = t29874 * t12830;
    let t42821 = 0.71137516589190373998e-2_f64 * t42820;
    let t42822 = 0.16598753870811087267e-1_f64 * t39805;
    let t42823 = 0.23712505529730124666e-2_f64 * t39808;
    let t42824 = 0.23712505529730124666e-2_f64 * t39811;
    let t42825 = t1358 * t12803;
    (t42814, t42815, t42816, t42817, t42821, t42822, t42823, t42824, t42825)
}
