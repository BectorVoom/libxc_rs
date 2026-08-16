//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1230/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1230(t10914: f64, t10915: f64, t32803: f64, t1: f64, t106: f64, t5745: f64, t787: f64, t191: f64, t5750: f64, t24784: f64, t2660: f64, t10827: f64, t2684: f64, t7354: f64) -> (f64, f64, f64, f64) {
    let t32806 = 0.21450293971110256001e2_f64 * t10914 * t10915 * t32803;
    let t32809 = t787 * t5745 * t1 * t106;
    let t32810 = t191 * t5750;
    let t32813 = 0.85801175884441024004e1_f64 * t32809 * t32810 * t32803;
    let t32815 = 0.21450293971110256002e1_f64 * t24784 * t2660;
    let t32817 = t2684 * t7354 * t10827;
    (t32806, t32813, t32815, t32817)
}
