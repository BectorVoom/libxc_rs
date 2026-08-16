//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1150/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1150(t560: f64, t922: f64, t811: f64, t839: f64, t694: f64, t9114: f64, t10761: f64, t15026: f64, t1680: f64, t2249: f64, t2254: f64, t24605: f64, t24623: f64, t32257: f64, t32264: f64, t33335: f64, t4818: f64, t4822: f64, t5399: f64, t567: f64, t643: f64, t7297: f64, t8034: f64, t8356: f64, t8372: f64, t9089: f64, t9096: f64, t9460: f64) -> (f64, f64, f64, f64) {
    let t36577 = t560 * t922;
    let t36611 = t560 * t811;
    let t36647 = t560 * t839;
    let t36684 = 6.0_f64 * t694 * t9114;
    let t36685 = -6.0_f64 * t10761 * t7297 * t9089 - t15026 * t567 * t643 - t1680 * t567 * t8356 - 2.0_f64 * t2249 * t5399 * t567 + 3.0_f64 * t2254 * t33335 * t567 + 4.0_f64 * t24605 * t9096 * t9460 + 6.0_f64 * t24623 * t7297 * t9460 + 12.0_f64 * t4818 * t8034 * t8372 + 6.0_f64 * t4822 * t8034 * t8372 + 2.0_f64 * t32257 + 6.0_f64 * t32264 - t36684;
    (t36577, t36611, t36647, t36685)
}
