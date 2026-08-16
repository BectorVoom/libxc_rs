//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1295/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1295(t29895: f64, t30517: f64, t29900: f64, t30524: f64, t30527: f64, t110075: f64, t30507: f64, t110082: f64, t110314: f64, t111101: f64, t111104: f64, t111109: f64, t111111: f64, t19517: f64, t30063: f64, t30175: f64, t5480: f64, t662: f64, t8137: f64, t8180: f64, t96715: f64) -> f64 {
    let t111763 = t29895 * t30517;
    let t111765 = t29900 * t30524;
    let t111767 = t29900 * t30527;
    let t111769 = t110075 * t30507;
    let t111772 = 5.0_f64 / 108.0_f64 * t8137 * t110314 * t5480 * t662 + 5.0_f64 / 18.0_f64 * t30175 * t30063 * t19517 + 3.0_f64 * t110082 * t8180 * t96715 - 2.0_f64 / 3.0_f64 * t111763 - 50.0_f64 / 27.0_f64 * t111765 + 10.0_f64 / 27.0_f64 * t111767 + 2.0_f64 * t111769 + 44.0_f64 / 9.0_f64 * t111101 - t111104 + t111109 - t111111;
    t111772
}
