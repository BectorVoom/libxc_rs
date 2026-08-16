//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 495/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk495(t2832: f64, t312: f64, t2649: f64, t2745: f64, t2750: f64, t2802: f64, t2845: f64, t2892: f64, t301: f64, t317: f64, t830: f64, t880: f64) -> (f64, f64) {
    let t2894 = t2832 * t312;
    let t2899 = -t2649 * t317 - t2745 * t317 - t2892 * t301 - 2.0_f64 * t830 * t880 - 4.0_f64 * t2750 - 2.0_f64 * t2802 + 4.0_f64 * t2845 + 2.0_f64 * t2894;
    (t2894, t2899)
}
