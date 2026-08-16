//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1395/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1395(t1873: f64, t96311: f64, t120826: f64, t120830: f64, t120835: f64, t120836: f64, t120838: f64, t120840: f64, t120848: f64, t120851: f64, t123261: f64, t31287: f64, t33192: f64, t577: f64) -> f64 {
    let t123306 = t96311 * t1873;
    let t123313 = 0.135e2_f64 * t120826 + 0.135e2_f64 * t123306 + t120830 + t31287 + t120835 + 27.0_f64 * t120836 + 27.0_f64 * t120838 + 27.0_f64 * t120840 + t33192 + t120848 + t120851 + 0.45e1_f64 * t123261 * t577;
    t123313
}
