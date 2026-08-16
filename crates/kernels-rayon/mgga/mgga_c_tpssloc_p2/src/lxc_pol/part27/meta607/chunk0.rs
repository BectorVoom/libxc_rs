//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2079/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2079(t1011: f64, t3120: f64, t23384: f64, t23650: f64, t10336: f64, t1920: f64, t1949: f64, t23323: f64, t6781: f64, t2966: f64, t6805: f64, t135: f64, t23631: f64, t6688: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82754 = t3120 * t1011;
    let t82789 = t23384 * t23650;
    let t82799 = 0.30461741978670859935e-2_f64 * t1920 * t10336 * t1949;
    let t82806 = t23323 * t6781;
    let t82809 = t1920 * t2966 * t6805;
    let t82822 = t23631 * t135 * t6688;
    (t82754, t82789, t82799, t82806, t82809, t82822)
}
