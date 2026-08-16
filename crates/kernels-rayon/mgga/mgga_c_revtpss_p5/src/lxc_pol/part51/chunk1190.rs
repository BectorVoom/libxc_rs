//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1190/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1190(t125362: f64, t1937: f64, t125365: f64, t33602: f64, t6993: f64, t127369: f64, t127371: f64, t127373: f64, t127375: f64, t127378: f64, t127384: f64, t127385: f64, t127393: f64, t127395: f64, t127397: f64, t127399: f64, t28030: f64, t32316: f64, t33903: f64, t4248: f64, t4292: f64, t5787: f64, t651: f64, t670: f64, t7007: f64, t8557: f64, t8565: f64) -> f64 {
    let t127401 = t125362 * t1937;
    let t127403 = t125365 * t1937;
    let t127405 = t33602 * t6993;
    let t127409 = -2.0_f64 * t33903 * t651 * t670 - 2.0_f64 * t4292 * t651 * t8557 - 4.0_f64 * t28030 * t7007 - 2.0_f64 * t32316 * t4248 + t5787 * t8565 - t127369 - t127371 - t127373 - t127375 - t127378 - t127384 - t127385 - 4.0_f64 * t127393 - 4.0_f64 * t127395 - 4.0_f64 * t127397 - 4.0_f64 * t127399 - 4.0_f64 * t127401 - 4.0_f64 * t127403 - 4.0_f64 * t127405;
    t127409
}
