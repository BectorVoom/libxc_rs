//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 578/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk578(t3821: f64, t676: f64, t27: f64, t89: f64, t2335: f64, t2338: f64, t2341: f64, t3688: f64, t3693: f64, t3697: f64, t3702: f64, t3707: f64, t3710: f64, t3715: f64, t3720: f64) -> (f64, f64, f64) {
    let t3822 = t676 * t3821;
    let t3824 = t89 * t27 * t3822;
    let t3826 = t2335 + t2338 / 54.0_f64 + t2341 / 18.0_f64 + t3688 / 54.0_f64 - t3693 / 27.0_f64 + t3697 / 18.0_f64 + t3702 / 9.0_f64 + t3707 / 9.0_f64 + t3710 / 18.0_f64 + t3715 / 18.0_f64 + t3720 / 3.0_f64 - t3824 / 6.0_f64;
    (t3822, t3824, t3826)
}
