//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 872/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk872(t107: f64, t2931: f64, t2021: f64, t1858: f64, t3038: f64, t787: f64, t2610: f64, t8669: f64) -> (f64, f64, f64, f64) {
    let t8748 = t2931 * t107;
    let t8749 = t2021 * t8748;
    let t8752 = t1858 * t3038;
    let t8753 = t787 * t8752;
    let t8756 = t2610 * t8669;
    (t8749, t8752, t8753, t8756)
}
