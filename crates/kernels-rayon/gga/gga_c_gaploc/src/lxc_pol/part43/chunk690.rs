//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 690/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk690(t12830: f64, t9074: f64, t12428: f64, t3152: f64, t988: f64, t2268: f64, t2765: f64, t3137: f64, t3145: f64, t8045: f64, t2798: f64, t3207: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12831 = t9074 * t12830;
    let t12832 = 0.71137516589190373998e-2_f64 * t12831;
    let t12833 = 0.71137516589190373998e-2_f64 * t12428;
    let t12834 = t3152 * t988;
    let t12836 = 0.28455006635676149599e-1_f64 * t2268 * t12834;
    let t12840 = t2765 * t3137;
    let t12842 = 0.85365019907028448797e-1_f64 * t2268 * t12840;
    let t12849 = 2.0_f64 * t8045 * t3145;
    let t12850 = t2798 * t3207;
    (t12832, t12833, t12834, t12836, t12840, t12842, t12849, t12850)
}
