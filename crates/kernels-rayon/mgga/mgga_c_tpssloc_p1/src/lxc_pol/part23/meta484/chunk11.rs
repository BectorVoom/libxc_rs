//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1483/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1483(t1420: f64, t1423: f64, t19368: f64, t19390: f64, t20217: f64, t20246: f64, t20255: f64, t20258: f64, t20261: f64, t2267: f64, t2274: f64, t39: f64, t39159: f64, t39168: f64, t39210: f64, t3981: f64, t3990: f64, t43: f64, t51: f64, t5398: f64, t5416: f64, t5421: f64, t5424: f64, t55: f64, t56: f64, t75836: f64, t75847: f64, t75912: f64, t78505: f64) -> f64 {
    let t79692 = 5.0_f64 / 162.0_f64 * t39 * t39159 * t75836 + 5.0_f64 / 6.0_f64 * t39 * t43 * t75912 + 20944.0_f64 / 81.0_f64 * t78505 * t56 + 12320.0_f64 / 81.0_f64 * t20246 * t1423 - 440.0_f64 / 9.0_f64 * t5416 * t5424 + 440.0_f64 / 27.0_f64 * t5416 * t5421 - 40.0_f64 / 81.0_f64 * t1420 * t20255 + 80.0_f64 / 9.0_f64 * t1420 * t20261 + 5.0_f64 / 162.0_f64 * t51 * t39168 * t75836 - 5.0_f64 / 6.0_f64 * t51 * t55 * t75912 - 5.0_f64 / 18.0_f64 * t39 * t19368 * t5398 + 5.0_f64 / 6.0_f64 * t39 * t2267 * t75847 + 10.0_f64 / 9.0_f64 * t39 * t3981 * t20217 - 80.0_f64 / 9.0_f64 * t1420 * t20258 + 5.0_f64 / 18.0_f64 * t51 * t19390 * t5398 + 5.0_f64 / 6.0_f64 * t51 * t2274 * t75847 + 10.0_f64 / 9.0_f64 * t51 * t3990 * t20217 - t39210;
    t79692
}
