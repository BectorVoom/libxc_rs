//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 880/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk880(t12826: f64, t6313: f64, t12840: f64, t6305: f64, t2268: f64, t3137: f64, t7930: f64, t2765: f64, t9152: f64, t39791: f64, t39794: f64, t39798: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42806 = 0.45528010617081839357e0_f64 * t6313 * t12826;
    let t42808 = 0.85365019907028448797e-1_f64 * t6305 * t12840;
    let t42811 = 0.85365019907028448797e-1_f64 * t2268 * t7930 * t3137;
    let t42814 = 0.85365019907028448797e-1_f64 * t2268 * t2765 * t9152;
    let t42815 = 0.23712505529730124666e-2_f64 * t39791;
    let t42816 = 0.23712505529730124666e-2_f64 * t39794;
    let t42817 = 0.23712505529730124666e-2_f64 * t39798;
    (t42806, t42808, t42811, t42814, t42815, t42816, t42817)
}
