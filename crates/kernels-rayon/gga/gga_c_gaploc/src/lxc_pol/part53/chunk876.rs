//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 876/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk876(t12834: f64, t6313: f64, t12826: f64, t6305: f64, t2268: f64, t26938: f64, t3133: f64, t31591: f64, t4261: f64, t9074: f64, t39731: f64, t2321: f64, t34600: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42708 = 0.37940008847568199465e-1_f64 * t6313 * t12834;
    let t42712 = 0.34146007962811379518e0_f64 * t6305 * t12826;
    let t42715 = 0.34146007962811379518e0_f64 * t2268 * t26938 * t3133;
    let t42717 = t9074 * t4261 * t31591;
    let t42718 = 0.47425011059460249332e-2_f64 * t42717;
    let t42719 = 0.23712505529730124666e-2_f64 * t39731;
    let t42721 = t9074 * t34600 * t2321;
    (t42708, t42712, t42715, t42718, t42719, t42721)
}
