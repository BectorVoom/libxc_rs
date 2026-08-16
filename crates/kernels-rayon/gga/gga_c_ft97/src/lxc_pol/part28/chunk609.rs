//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 609/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk609(t25802: f64, t73: f64, t22632: f64, t5598: f64, t6445: f64, t22652: f64, t938: f64, t6427: f64, t7839: f64, t25653: f64, t5540: f64, t22743: f64, t25774: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25803 = t73 * t25802;
    let t25813 = t5598 * t22632 * t6445;
    let t25816 = t22652 * t938;
    let t25820 = t6427 * t7839;
    let t25826 = t5540 * t25653;
    let t25829 = t5540 * t25802;
    let t25832 = t22743 * t25774;
    (t25803, t25813, t25816, t25820, t25826, t25829, t25832)
}
