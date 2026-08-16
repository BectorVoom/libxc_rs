//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1435/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1435(t115545: f64, t26331: f64, t26333: f64, t114225: f64, t115572: f64, t120561: f64, t120566: f64, t120569: f64, t1375: f64, t1807: f64, t2016: f64, t22670: f64, t26472: f64, t31584: f64, t31601: f64, t3887: f64, t5210: f64, t5215: f64, t568: f64, t7194: f64, t7213: f64, t7749: f64, t7925: f64, t8617: f64, t93313: f64) -> f64 {
    let t122304 = t26331 * t115545 * t26333;
    let t122319 = 0.41123351671205660912e-2_f64 * t115572 + t114225 - t7194 * t26472 + 0.49348022005446793095e-1_f64 * t122304 - t120561 - t120566 - t93313 * t2016 + t5210 * t8617 * t568 + t1807 * t31584 * t568 + 2.0_f64 * t22670 * t7925 + 2.0_f64 * t1375 * t3887 * t7213 * t7749 + t120569 + 2.0_f64 * t5215 * t31601;
    t122319
}
