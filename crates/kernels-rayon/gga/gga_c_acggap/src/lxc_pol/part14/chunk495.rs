//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 495/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk495(t203: f64, t328: f64, t202: f64, t2607: f64, t4: f64, t11: f64, t2: f64, t39: f64, t2673: f64, t672: f64, t210: f64, t21: f64, t5: f64, t575: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2676 = t203 * t328;
    let t2677 = t202 * t2676;
    let t2679 = t4 * t2607;
    let t2681 = 1.0_f64/pow_3_2(t11);
    let t2682 = t2681 * t2;
    let t2683 = t2682 * t39;
    let t2685 = t672 * t2673;
    let t2687 = t210 * t2676;
    let t2690 = t21 * t5 * t575;
    (t2677, t2679, t2683, t2685, t2687, t2690)
}
