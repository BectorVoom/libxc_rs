//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 886/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk886(t161: f64, t2931: f64, t1854: f64, t1858: f64, t3487: f64, t734: f64, t7289: f64, t8769: f64, t8773: f64, t1845: f64, t5396: f64, t8756: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8867 = t2931 * t161;
    let t8868 = t8867 * t1854;
    let t8871 = t1858 * t3487;
    let t8872 = t8871 * t734;
    let t8875 = t7289 * t8769;
    let t8878 = t8773 * t161;
    let t8879 = t8878 * t1845;
    let t8882 = t5396 * t8756;
    (t8868, t8871, t8872, t8875, t8878, t8879, t8882)
}
