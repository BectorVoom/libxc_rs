//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1178/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1178(t142995: f64, t1466: f64, t1506: f64, t154221: f64, t193: f64, t28978: f64, t29035: f64, t34022: f64, t34025: f64, t34262: f64, t34312: f64, t34326: f64, t34330: f64, t36273: f64, t4129: f64, t6222: f64, t6963: f64, t7024: f64, t7581: f64, t830: f64) -> f64 {
    let t155009 = t6963 * t34326 / 3.0_f64 - t830 * t36273 + t6963 * t34022 - 2.0_f64 / 3.0_f64 * t6963 * t34025 - 2.0_f64 * t154221 + t6963 * t34262 / 6.0_f64 + t6963 * t34330 / 3.0_f64 + t34312 * t7024 / 6.0_f64 - t142995 / 18.0_f64 - 2.0_f64 / 3.0_f64 * t1466 * t193 * t6222 * t1506 * t4129 - t7581 * t28978 / 3.0_f64 - t7581 * t29035 / 3.0_f64;
    t155009
}
