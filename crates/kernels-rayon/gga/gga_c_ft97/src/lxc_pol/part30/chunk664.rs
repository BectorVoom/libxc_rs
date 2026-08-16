//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 664/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk664(t213: f64, t231: f64, t811: f64, t6819: f64, t2724: f64, t39: f64, t5585: f64, t4113: f64) -> (f64, f64, f64, f64) {
    let t28654 = t231 * t213 * t811;
    let t28655 = t6819 * t28654;
    let t28658 = t2724 * t39;
    let t28659 = t28658 * t5585;
    let t28660 = t4113 * t28659;
    (t28655, t28658, t28659, t28660)
}
