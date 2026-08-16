//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1302/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1302(t40: f64, t52: f64, t16549: f64, t20217: f64, t2433: f64, t40632: f64, t4080: f64, t5398: f64, t73: f64, t75836: f64, t75847: f64, t75912: f64, t16563: f64, t2440: f64, t40647: f64, t4087: f64, t76: f64, zeta_threshold: f64) -> (f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t75916 = piecewise3(t146, 0.0_f64, 40.0_f64 / 81.0_f64 * t40632 * t75836 - 16.0_f64 / 9.0_f64 * t16549 * t5398 + 4.0_f64 / 3.0_f64 * t2433 * t75847 + 16.0_f64 / 9.0_f64 * t4080 * t20217 + 4.0_f64 / 3.0_f64 * t73 * t75912);
    let t75928 = piecewise3(t150, 0.0_f64, 40.0_f64 / 81.0_f64 * t40647 * t75836 + 16.0_f64 / 9.0_f64 * t16563 * t5398 + 4.0_f64 / 3.0_f64 * t2440 * t75847 + 16.0_f64 / 9.0_f64 * t4087 * t20217 - 4.0_f64 / 3.0_f64 * t76 * t75912);
    (t75916, t75928)
}
