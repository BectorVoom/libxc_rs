//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1291/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1291(t120849: f64, t8319: f64, t114475: f64, t114495: f64, t120815: f64, t120818: f64, t120820: f64, t120823: f64, t120826: f64, t120830: f64, t120835: f64, t120836: f64, t120838: f64, t120840: f64, t120842: f64, t120848: f64, t1458: f64, t31267: f64, t31287: f64, t33192: f64, t4072: f64, t5376: f64, t671: f64) -> f64 {
    let t120851 = 27.0_f64 * t120849 * t8319;
    let t120852 = 27.0_f64 * t120815 + t120818 + t120820 + t120823 + 0.135e2_f64 * t31267 * t4072 + 27.0_f64 * t120826 + t120830 + 27.0_f64 * t114495 * t5376 + t31287 + t120835 + 54.0_f64 * t120836 + 54.0_f64 * t120838 + 54.0_f64 * t120840 + 0.135e2_f64 * t120842 * t671 + t33192 + 0.135e2_f64 * t114475 * t1458 + t120848 + t120851;
    t120852
}
