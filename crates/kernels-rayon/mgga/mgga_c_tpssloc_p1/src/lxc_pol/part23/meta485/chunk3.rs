//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1487/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1487(t5464: f64, t5488: f64, t5468: f64, t5396: f64, t5480: f64, t5484: f64, t75910: f64, t100: f64, t103: f64, t104: f64, t1447: f64, t1450: f64, t19488: f64, t19513: f64, t20245: f64, t20318: f64, t20322: f64, t20332: f64, t20335: f64, t20338: f64, t20339: f64, t2341: f64, t2349: f64, t4049: f64, t4059: f64, t45460: f64, t45496: f64, t5475: f64, t5481: f64, t5485: f64, t92: f64, t95: f64, tau1: f64) -> (f64, f64, f64) {
    let t79748 = t5464 * t5464;
    let t79755 = t5488 * t5488;
    let t79761 = t5468 * t5468;
    let t79768 = t5396 * t5396;
    let t79781 = t5480 * t5480;
    let t79788 = t5484 * t5484;
    let t79795 = 12.0_f64 * t75910;
    let t79812 = 40.0_f64 / 81.0_f64 * t92 * t45496 * t79761 - 20.0_f64 / 9.0_f64 * t92 * t19488 * t5396 + 10.0_f64 / 3.0_f64 * t92 * t2341 * t79768 + 40.0_f64 / 9.0_f64 * t92 * t4049 * t20318 + 800.0_f64 / 27.0_f64 * t5475 * t5481 + 200.0_f64 / 81.0_f64 * t1447 * t20332 - 200.0_f64 / 9.0_f64 * t1447 * t20335 + 40.0_f64 / 81.0_f64 * t100 * t45460 * t79781 - 20.0_f64 / 9.0_f64 * t100 * t19513 * t5484 + 10.0_f64 / 3.0_f64 * t100 * t2349 * t79788 + 40.0_f64 / 9.0_f64 * t100 * t4059 * t20338 + 5.0_f64 / 3.0_f64 * t92 * t95 * t79795 + 6160.0_f64 / 81.0_f64 * tau1 * t20245 * t104 - 8800.0_f64 / 81.0_f64 * t20322 * t1450 + 400.0_f64 / 9.0_f64 * t5475 * t5485 - 100.0_f64 / 9.0_f64 * t1447 * t20339 - 5.0_f64 / 3.0_f64 * t100 * t103 * t79795;
    (t79748, t79755, t79812)
}
