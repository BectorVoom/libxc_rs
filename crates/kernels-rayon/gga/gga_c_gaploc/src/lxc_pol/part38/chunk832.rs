//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 832/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk832(t44318: f64, t35893: f64, t4261: f64, t9074: f64, t11280: f64, t2326: f64, t2268: f64, t2440: f64, t3518: f64, t44268: f64, t447: f64, t13319: f64, t6313: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44319 = 0.35568758294595186999e-2_f64 * t44318;
    let t44321 = t9074 * t4261 * t35893;
    let t44322 = 0.23712505529730124666e-2_f64 * t44321;
    let t44324 = t9074 * t11280 * t2326;
    let t44325 = 0.82993769354055436331e-2_f64 * t44324;
    let t44328 = 0.28455006635676149599e-1_f64 * t2268 * t2440 * t3518;
    let t44329 = t44268 * t447;
    let t44334 = 0.37940008847568199465e-1_f64 * t6313 * t13319;
    (t44319, t44322, t44325, t44328, t44329, t44334)
}
