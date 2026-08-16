//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1180/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1180(t10697: f64, t142653: f64, t143432: f64, t1466: f64, t154842: f64, t193: f64, t2404: f64, t25412: f64, t28868: f64, t28934: f64, t28940: f64, t28946: f64, t28992: f64, t33966: f64, t36011: f64, t36060: f64, t36063: f64, t4309: f64, t44601: f64, t6216: f64, t6386: f64, t683: f64, t684: f64, t7114: f64, t7585: f64, t875: f64) -> f64 {
    let t155066 = t1466 * t193 * t33966 * t28868 - t1466 * t193 * t7585 * t4309 / 3.0_f64 - 12.0_f64 * t10697 * t36060 * t875 + 48.0_f64 * t44601 * t36063 * t875 - 24.0_f64 * t10697 * t7114 * t6386 + 2.0_f64 / 9.0_f64 * t6216 * t25412 * t36011 * t684 - t6216 * t142653 * t28934 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t6216 * t25412 * t28992 - 2.0_f64 / 9.0_f64 * t6216 * t683 * t7585 * t28940 + 2.0_f64 / 27.0_f64 * t6216 * t2404 * t7585 * t28946 + t143432 / 9.0_f64 - 4.0_f64 * t154842;
    t155066
}
