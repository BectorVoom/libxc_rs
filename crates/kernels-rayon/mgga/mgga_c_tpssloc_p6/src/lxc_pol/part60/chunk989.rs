//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 989/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk989(t1799: f64, t7752: f64, t28030: f64, t8327: f64, t32677: f64, t7458: f64, t20162: f64, t8326: f64, t28893: f64, t33194: f64, t16524: f64, t33193: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t127553 = t1799 * t7752;
    let t127560 = 2.0_f64 * t28030 * t8327;
    let t127562 = 4.0_f64 * t7458 * t32677;
    let t127601 = 0.135e2_f64 * t20162 * t8326;
    let t127603 = 27.0_f64 * t28893 * t8326;
    let t127606 = 54.0_f64 * t33194;
    let t127608 = 54.0_f64 * t16524 * t33193;
    (t127553, t127560, t127562, t127601, t127603, t127606, t127608)
}
