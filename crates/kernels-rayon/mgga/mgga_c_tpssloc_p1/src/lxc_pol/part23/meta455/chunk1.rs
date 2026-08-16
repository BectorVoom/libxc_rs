//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1314/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1314(t5527: f64, t5544: f64, t1504: f64, t1506: f64, t16729: f64, t16736: f64, t20800: f64, t20835: f64, t20843: f64, t20846: f64, t20849: f64, t225: f64, t228: f64, t230: f64, t2671: f64, t41315: f64, t4225: f64, t4226: f64, t5601: f64, t5605: f64, t5608: f64, t75978: f64, t76006: f64, t76007: f64, t76009: f64, t76010: f64, t76013: f64, t76014: f64, t76021: f64, t76038: f64, t824: f64) -> (f64, f64, f64) {
    let t76056 = t5527 * t5527;
    let t76063 = t5544 * t5544;
    let t76073 = -(t76006 + t76007 + t76009 + t76010 + t76013 + t76014 + t76021 + t76038) * t225 * t230 + 12.0_f64 * t20835 * t1506 - 72.0_f64 * t5601 * t5605 + 18.0_f64 * t5601 * t5608 + 240.0_f64 * t1504 * t20843 - 144.0_f64 * t16729 * t20846 + 12.0_f64 * t1504 * t20849 - 360.0_f64 * t228 * t41315 * t76056 + 360.0_f64 * t4225 * t16736 * t5544 - 36.0_f64 * t228 * t2671 * t76063 - 48.0_f64 * t4225 * t4226 * t20800 + 3.0_f64 * t228 * t824 * t75978;
    (t76056, t76063, t76073)
}
