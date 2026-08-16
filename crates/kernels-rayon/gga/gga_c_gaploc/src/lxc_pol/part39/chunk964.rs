//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 964/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk964(t2268: f64, t32005: f64, t894: f64, t41596: f64, t426: f64, t535: f64, t39671: f64, t39674: f64, t39677: f64, t39679: f64, t39681: f64, t12837: f64, t6305: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42597 = 0.56910013271352299198e-1_f64 * t2268 * t894 * t32005;
    let t42601 = 0.28455006635676149599e-1_f64 * t2268 * t535 * t41596 * t426;
    let t42602 = 0.284550066356761496e-1_f64 * t39671;
    let t42603 = 0.142275033178380748e-1_f64 * t39674;
    let t42604 = 0.23712505529730124666e-2_f64 * t39677;
    let t42605 = 0.47425011059460249332e-2_f64 * t39679;
    let t42606 = 0.71137516589190373998e-2_f64 * t39681;
    let t42607 = t6305 * t12837;
    (t42597, t42601, t42602, t42603, t42604, t42605, t42606, t42607)
}
