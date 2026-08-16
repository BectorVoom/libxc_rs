//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 911/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk911(t3281: f64, t454: f64, t1822: f64, t8232: f64, t1893: f64, t110: f64, t1825: f64, t1866: f64, t1901: f64, t38079: f64, t38103: f64, t38254: f64, t38711: f64, t38732: f64, t432: f64, t446: f64, t447: f64, t452: f64, t488: f64, t492: f64, t8183: f64, t8549: f64, t8558: f64, t8590: f64) -> f64 {
    let t38734 = t3281 * t454;
    let t38740 = t8232 * t1822;
    let t38742 = t8232 * t1893;
    let t38744 = 4.0_f64 * t446 * t452 * t1825 * t8549 - 8.0_f64 / 3.0_f64 * t1901 * t38711 * t8558 - 2.0_f64 / 9.0_f64 * t446 * t1866 * t110 * t38079 + 2.0_f64 / 3.0_f64 * t446 * t447 * t110 * t38103 + 4.0_f64 / 3.0_f64 * t446 * t452 * t488 * t8183 * t492 - t446 * t452 * t110 * t38254 / 3.0_f64 + 112.0_f64 / 81.0_f64 * t38732 + 112.0_f64 / 81.0_f64 * t38734 - 4.0_f64 / 3.0_f64 * t446 * t452 * t8590 * t432 - 8.0_f64 / 9.0_f64 * t38740 - 16.0_f64 / 9.0_f64 * t38742;
    t38744
}
