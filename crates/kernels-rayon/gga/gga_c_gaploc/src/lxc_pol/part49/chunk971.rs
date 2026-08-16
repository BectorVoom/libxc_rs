//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 971/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk971(t42717: f64, t39731: f64, t2321: f64, t34600: f64, t9074: f64, t12820: f64, t484: f64, t1063: f64, t31308: f64, t7937: f64, t2268: f64, t31399: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42718 = 0.47425011059460249332e-2_f64 * t42717;
    let t42719 = 0.23712505529730124666e-2_f64 * t39731;
    let t42721 = t9074 * t34600 * t2321;
    let t42722 = 0.23712505529730124666e-2_f64 * t42721;
    let t42726 = t484 * t12820;
    let t42730 = 0.34146007962811379518e0_f64 * t1063 * t7937 * t31308;
    let t42733 = 0.68292015925622759036e0_f64 * t2268 * t7937 * t31399;
    (t42718, t42719, t42722, t42726, t42730, t42733)
}
