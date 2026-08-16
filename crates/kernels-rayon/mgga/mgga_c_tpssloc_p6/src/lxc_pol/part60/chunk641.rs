//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 641/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk641(t3: f64, t8110: f64, t1458: f64, t577: f64, t7423: f64, t7768: f64, t7771: f64, t7773: f64, t33: f64, t68: f64, t69: f64, t79: f64) -> (f64, f64, f64, f64, f64) {
    let t8111 = t3 * t8110;
    let t8119 = 0.45e1_f64 * t8110 * t577 + 0.135e2_f64 * t7423 * t1458 + t7768 + t7771 + t7773;
    let t8301 = t33 * t33;
    let t8306 = 1.0_f64 / t69 / t68;
    let t8307 = t79 * t79;
    (t8111, t8119, t8301, t8306, t8307)
}
