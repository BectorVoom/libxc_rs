//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1087/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1087(t13287: f64, t31195: f64, t35340: f64, t1980: f64, t34487: f64, t7476: f64, t2118: f64, t5082: f64, t142: f64, t2060: f64, t4838: f64, t604: f64) -> (f64, f64, f64, f64) {
    let t35342 = t31195 * t13287 * t35340;
    let t35348 = t1980 * t7476 * t34487;
    let t35350 = t2118 * t5082;
    let t35357 = t2060 * t142 * t604 * t4838;
    (t35342, t35348, t35350, t35357)
}
