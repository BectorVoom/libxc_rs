//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 917/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk917(t9823: f64, t9824: f64, t165: f64, t2530: f64, t161: f64, t2685: f64, t2684: f64, t2465: f64, t2581: f64, t2464: f64, t3311: f64, t7416: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9826 = 0.29792074959875355558e-1_f64 * t9823 * t9824;
    let t9828 = t165 * t2530;
    let t9829 = t161 * t9828;
    let t9830 = t2685 * t9829;
    let t9831 = t2684 * t9830;
    let t9833 = t2465 * t2581;
    let t9834 = t2464 * t9833;
    let t9835 = t2684 * t9834;
    let t9837 = t7416 * t3311;
    (t9826, t9829, t9830, t9831, t9833, t9834, t9835, t9837)
}
