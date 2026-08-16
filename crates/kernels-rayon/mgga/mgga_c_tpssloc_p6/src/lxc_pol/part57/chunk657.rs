//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 657/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk657(t2020: f64, t8607: f64, t2085: f64, t225: f64, t567: f64, t214: f64, t1985: f64, t8463: f64, t8468: f64) -> (f64, f64, f64, f64, f64) {
    let t8608 = t8607 * t2020;
    let t8611 = t2085 * t225 * t567;
    let t8612 = t214 * t8611;
    let t8613 = t1985 * t8612;
    let t8617 = 0.16149102437656156341e-2_f64 * t8463 + t8468 / 768.0_f64;
    (t8608, t8611, t8612, t8613, t8617)
}
