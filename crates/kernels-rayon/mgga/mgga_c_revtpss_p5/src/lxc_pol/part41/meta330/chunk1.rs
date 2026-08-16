//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1124/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1124(t14732: f64, t2484: f64, t2652: f64, t4435: f64, t4343: f64, t854: f64, t236: f64, t807: f64, t221: f64, t4433: f64, t10703: f64, t2674: f64) -> (f64, f64, f64, f64) {
    let t14734 = 0.25410001404642664112e-4_f64 * t2484 * t14732;
    let t14736 = 0.40015750243531754508e-1_f64 * t2652 * t4435;
    let t14741 = t854 * t4343;
    let t14742 = t236 * t14741;
    let t14744 = 0.57165357490759649296e-4_f64 * t807 * t14742;
    let t14756 = t221 * t4433;
    let t14757 = t10703 * t14756;
    let t14759 = 0.50820002809285328225e-3_f64 * t2674 * t14757;
    (t14734, t14736, t14744, t14759)
}
