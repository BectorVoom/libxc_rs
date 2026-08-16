//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 970/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk970(t12810: f64, t6305: f64, t2268: f64, t2756: f64, t3148: f64, t12834: f64, t6313: f64, t12837: f64, t12826: f64, t26938: f64, t3133: f64, t31591: f64, t4261: f64, t9074: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42703 = 0.28455006635676149599e-1_f64 * t6305 * t12810;
    let t42706 = 0.28455006635676149599e-1_f64 * t2268 * t3148 * t2756;
    let t42708 = 0.37940008847568199465e-1_f64 * t6313 * t12834;
    let t42709 = t6313 * t12837;
    let t42712 = 0.34146007962811379518e0_f64 * t6305 * t12826;
    let t42715 = 0.34146007962811379518e0_f64 * t2268 * t26938 * t3133;
    let t42717 = t9074 * t4261 * t31591;
    (t42703, t42706, t42708, t42709, t42712, t42715, t42717)
}
