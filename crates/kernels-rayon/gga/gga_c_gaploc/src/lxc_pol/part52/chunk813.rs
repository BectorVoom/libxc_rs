//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 813/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk813(t44324: f64, t2268: f64, t2440: f64, t3518: f64, t13319: f64, t6313: f64, t6305: f64, t13313: f64, t38184: f64, t888: f64, t2349: f64, t3565: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44325 = 0.82993769354055436331e-2_f64 * t44324;
    let t44328 = 0.28455006635676149599e-1_f64 * t2268 * t2440 * t3518;
    let t44334 = 0.37940008847568199465e-1_f64 * t6313 * t13319;
    let t44336 = 0.28455006635676149599e-1_f64 * t6305 * t13319;
    let t44350 = 0.85365019907028448797e-1_f64 * t6305 * t13313;
    let t44355 = 0.85365019907028448797e-1_f64 * t2268 * t38184 * t888;
    let t44358 = 0.19918504644973304719e0_f64 * t2268 * t3565 * t2349;
    (t44325, t44328, t44334, t44336, t44350, t44355, t44358)
}
