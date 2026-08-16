//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1181/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1181(t1218: f64, t1253: f64, t143538: f64, t1466: f64, t154242: f64, t154492: f64, t154794: f64, t154833: f64, t193: f64, t24964: f64, t28955: f64, t28985: f64, t28993: f64, t33808: f64, t34251: f64, t34260: f64, t35802: f64, t36011: f64, t4027: f64, t6210: f64, t7581: f64, t7684: f64) -> f64 {
    let t155092 = -4.0_f64 * t154833 + t143538 / 9.0_f64 - t4027 * t7684 - 4.0_f64 * t154242 - 2.0_f64 * t154492 - 2.0_f64 / 3.0_f64 * t1466 * t193 * t24964 * t36011 - 2.0_f64 * t154794 - t1218 * t34251 + t7581 * t28955 / 6.0_f64 - t33808 * t28993 / 18.0_f64 - 2.0_f64 / 3.0_f64 * t6210 * t35802 - 2.0_f64 / 3.0_f64 * t1466 * t193 * t24964 * t28985 + t1466 * t193 * t34260 * t1253 / 6.0_f64;
    t155092
}
