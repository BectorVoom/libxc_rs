//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1393/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1393(t12048: f64, t1580: f64, t30788: f64, t30791: f64, t30793: f64, t30805: f64, t34556: f64, t34566: f64, t34573: f64, t34576: f64, t34579: f64, t34581: f64, t34583: f64, t34586: f64, t34588: f64, t34592: f64, t34595: f64) -> f64 {
    let t38648 = t34556 + 0.23005755572352449806e2_f64 * t1580 * t12048 + t30788 + t30791 - 0.53964118009221795842e0_f64 * t30793 - t34566 - t34573 + t34576 + t34579 + t34581 - t34583 - t34586 + t34588 - t34592 + t34595 + t30805;
    t38648
}
