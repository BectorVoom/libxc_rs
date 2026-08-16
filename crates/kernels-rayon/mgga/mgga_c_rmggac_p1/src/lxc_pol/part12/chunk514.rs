//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 514/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk514(t446: f64, t5656: f64, t1487: f64, t998: f64, t472: f64, t5527: f64, t1201: f64, t1206: f64, t1209: f64, t1480: f64, t1486: f64, t1488: f64, t1491: f64, t206: f64, t207: f64, t470: f64, t473: f64, t5637: f64, t5647: f64, t5653: f64, t600: f64, t602: f64) -> f64 {
    let t5657 = t5656 * t446;
    let t5660 = t1487 * t998;
    let t5663 = t472 * t5527;
    let t5666 = 3.0_f64 * t1201 * t602 - 12.0_f64 * t1206 * t600 + 3.0_f64 * t1209 * t600 + 6.0_f64 * t1480 * t473 + 60.0_f64 * t1486 * t5653 - 24.0_f64 * t1486 * t5657 - 12.0_f64 * t1486 * t5660 - 24.0_f64 * t1488 * t5647 + 6.0_f64 * t1491 * t470 + 3.0_f64 * t206 * t5663 - t207 * t5637;
    t5666
}
