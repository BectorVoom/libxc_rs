//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 987/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk987(t10717: f64, t2580: f64, t2508: f64, t10677: f64, t701: f64) -> (f64, f64, f64) {
    let t10718 = t2580 * t10717;
    let t10720 = 0.15381052460284448567e-1_f64 * t2508 * t10718;
    let t10721 = t10677 * t701;
    (t10718, t10720, t10721)
}
