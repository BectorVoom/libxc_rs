//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1088/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1088(t1091: f64, t142455: f64, t142460: f64, t142653: f64, t142946: f64, t2404: f64, t25412: f64, t2665: f64, t28934: f64, t28935: f64, t28940: f64, t28941: f64, t28946: f64, t28947: f64, t28950: f64, t28951: f64, t28986: f64, t33808: f64, t6216: f64, t683: f64, t7612: f64) -> f64 {
    let t152530 = -t33808 * t28947 / 27.0_f64 + t33808 * t28951 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t6216 * t25412 * t28986 - t6216 * t2665 * t142460 * t1091 / 9.0_f64 - t6216 * t2665 * t142455 * t1091 / 9.0_f64 + t33808 * t28935 / 9.0_f64 + t33808 * t28941 / 9.0_f64 - t6216 * t2404 * t7612 * t28946 / 27.0_f64 + t6216 * t142946 * t28934 / 9.0_f64 + t6216 * t683 * t7612 * t28940 / 9.0_f64 + t6216 * t142946 * t28950 / 9.0_f64 - t6216 * t142653 * t28950 / 3.0_f64;
    t152530
}
