//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1403/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1403(t34800: f64, t10241: f64, t9448: f64, t15482: f64, t20560: f64, t9439: f64, t20555: f64, t10543: f64, t1407: f64, t1429: f64, t2365: f64, t2366: f64, t25729: f64) -> (f64, f64, f64, f64, f64) {
    let t34801 = 0.89376224879626066674e-1_f64 * t34800;
    let t34814 = t9448 * t10241;
    let t34817 = 0.5680433474654925878e0_f64 * t20560 * t15482 * t34814;
    let t34818 = t9439 * t10241;
    let t34821 = 0.22721733898619703511e0_f64 * t20555 * t15482 * t34818;
    let t34822 = t1407 * t10543;
    let t34823 = 0.51123901271894332902e0_f64 * t34822;
    let t34826 = t1429 * t2365 * t2366 * t25729;
    (t34801, t34817, t34821, t34823, t34826)
}
