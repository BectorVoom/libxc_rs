//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1050/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1050(t25709: f64, t32152: f64, t25718: f64, t22563: f64, t52: f64, t7837: f64, t100483: f64, t136474: f64, t136475: f64, t137047: f64, t22513: f64, t22837: f64, t25649: f64, t25653: f64, t25699: f64, t25704: f64, t25755: f64, t25760: f64, t25829: f64, t25832: f64, t32151: f64, t32250: f64, t44965: f64, t58: f64, t92354: f64, t929: f64, t93047: f64, t93157: f64) -> (f64, f64, f64) {
    let t145416 = t32152 * t25709;
    let t145419 = t32152 * t25718;
    let t145436 = t7837 * t22563 * t52;
    let t145447 = -0.24041029937711879614e-5_f64 * t44965 * t32250 * t22837 * t58 * t929 - 0.22705522127871165896e-3_f64 * t22513 * t145416 + 0.15137014751914110597e-3_f64 * t22513 * t145419 + 0.23022991505793434254e-7_f64 * t100483 * t92354 * t32151 * t25755 - 0.22705522127871165896e-3_f64 * t93047 * t32152 * t25760 + 0.22705522127871165896e-3_f64 * t93157 * t32152 * t25699 - 0.22705522127871165896e-3_f64 * t93047 * t32152 * t25704 - 0.20676097475611486194e-4_f64 * t145436 * t25829 - 0.68872808489893002037e-5_f64 * t145436 * t25832 - 0.13649345781532662579e-3_f64 * t137047 * t136475 * t25649 + 0.20474018672298993868e-3_f64 * t136474 * t136475 * t25653;
    (t145416, t145419, t145447)
}
