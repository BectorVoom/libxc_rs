//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1222/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1222(t31537: f64, t7467: f64, t120112: f64, t114418: f64, t1983: f64, t7687: f64, t15868: f64, t8489: f64, t22751: f64, t32731: f64, t22633: f64, t22635: f64, t31099: f64, t5187: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120140 = 4.0_f64 * t31537 * t7467;
    let t120165 = 2.0_f64 * t120112;
    let t120171 = 3.0_f64 * t1983 * t114418 * t7687;
    let t120176 = t1983 * t8489 * t15868;
    let t120179 = t22751 * t32731;
    let t120180 = 0.76763589786250567037e-1_f64 * t120179;
    let t120184 = 0.3289868133696452873e-1_f64 * t22633 * t22635 * t31099 * t5187;
    (t120140, t120165, t120171, t120176, t120180, t120184)
}
