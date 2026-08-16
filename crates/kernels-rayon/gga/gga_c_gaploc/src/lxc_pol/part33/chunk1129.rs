//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1129/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1129(t1358: f64, t9205: f64, t20513: f64, t4261: f64, t9074: f64, t20521: f64, t1365: f64, t20358: f64, t6525: f64, t19532: f64, t20370: f64, t2300: f64, t23983: f64, t6455: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30120 = 0.18970004423784099732e-1_f64 * t1358 * t9205;
    let t30123 = 0.94850022118920498664e-2_f64 * t9074 * t4261 * t20513;
    let t30126 = 0.47425011059460249332e-2_f64 * t9074 * t4261 * t20521;
    let t30129 = 0.23712505529730124666e-2_f64 * t6525 * t1365 * t20358;
    let t30132 = 0.142275033178380748e-1_f64 * t9074 * t19532 * t20370;
    let t30135 = 0.47425011059460249332e-2_f64 * t23983 * t2300 * t6455;
    (t30120, t30123, t30126, t30129, t30132, t30135)
}
