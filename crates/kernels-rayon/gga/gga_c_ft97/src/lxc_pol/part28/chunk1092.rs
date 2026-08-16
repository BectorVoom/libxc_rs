//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1092/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1092(t1882: f64, t34750: f64, t34714: f64, t34562: f64, t38659: f64, t32419: f64, t46565: f64, t138361: f64, t138367: f64, t144813: f64, t145922: f64, t1901: f64, t26390: f64, t32077: f64, t3238: f64, t32571: f64, t34632: f64, t34677: f64, t34768: f64, t379: f64, t446: f64, t452: f64, t46874: f64, t5710: f64, t83: f64, t8411: f64, t8466: f64, t8506: f64, t8557: f64, t986: f64) -> (f64, f64, f64, f64, f64) {
    let t146923 = t1882 * t34750;
    let t146929 = t1882 * t34714;
    let t146937 = t38659 * t34562;
    let t146972 = t46565 * t32419;
    let t146976 = 2.0_f64 / 3.0_f64 * t1901 * t46874 * t144813 - t1901 * t8557 * t34768 * t379 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t8506 * t34677 - 2.0_f64 * t446 * t8411 * t986 * t32077 + t138361 - 2.0_f64 / 9.0_f64 * t138367 - 2.0_f64 / 3.0_f64 * t446 * t452 * t8466 * t34632 - t446 * t83 * t145922 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t452 * t3238 * t32571 + 2.0_f64 / 3.0_f64 * t446 * t452 * t5710 * t26390 - 2.0_f64 * t446 * t83 * t146972;
    (t146923, t146929, t146937, t146972, t146976)
}
