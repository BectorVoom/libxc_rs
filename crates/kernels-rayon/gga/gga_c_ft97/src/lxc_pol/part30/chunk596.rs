//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 596/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk596(t6050: f64, t6828: f64, t1417: f64, t1127: f64, t218: f64, t709: f64, t25057: f64, t6776: f64, t694: f64, t226: f64, t6762: f64, t3817: f64, t6018: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27720 = t6828 * t6050;
    let t27721 = t1417 * t27720;
    let t27723 = t218 * t1127;
    let t27724 = t27723 * t709;
    let t27725 = t25057 * t27724;
    let t27729 = t694 * t6776;
    let t27730 = t27729 * t709;
    let t27733 = t6762 * t226;
    let t27736 = t6018 * t3817;
    (t27720, t27721, t27724, t27725, t27729, t27730, t27733, t27736)
}
