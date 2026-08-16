//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1212/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1212(t18670: f64, t5489: f64, t1791: f64, t18351: f64, t5492: f64, t5791: f64, t1675: f64, t1792: f64, t18305: f64, t18338: f64, t18347: f64, t18350: f64, t18356: f64, t18360: f64, t18363: f64, t18366: f64, t18648: f64, t18649: f64, t18652: f64, t18661: f64, t18663: f64, t18666: f64, t5483: f64, t5785: f64, t5794: f64) -> (f64, f64, f64, f64) {
    let t18671 = t18670 * t5489;
    let t18673 = t1791 * t18351;
    let t18676 = t5492 * t5791;
    let t18678 = -10.0_f64 / 3.0_f64 * t5785 * t18356 - 5.0_f64 / 3.0_f64 * t5785 * t18360 - 2.0_f64 / 3.0_f64 * t18363 * t1792 - 2.0_f64 / 3.0_f64 * t18366 * t1792 - 4.0_f64 / 3.0_f64 * t5492 * t5794 + t18648 - 10.0_f64 / 3.0_f64 * t18649 * t5489 - 16.0_f64 / 9.0_f64 * t18652 - 4.0_f64 / 3.0_f64 * t18338 * t1792 + t18305 * t1792 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t5483 * t5794 - 16.0_f64 / 9.0_f64 * t18661 + t1675 * t18663 / 3.0_f64 + 10.0_f64 * t18666 * t18347 + 80.0_f64 / 9.0_f64 * t18671 + 20.0_f64 / 3.0_f64 * t18350 * t18673 + 32.0_f64 / 9.0_f64 * t18676;
    (t18671, t18673, t18676, t18678)
}
