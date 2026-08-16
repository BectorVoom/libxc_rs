//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2822/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2822(t2832: f64, t890: f64, t11064: f64, t14353: f64, t14436: f64, t1940: f64, t2403: f64, t2408: f64, t2430: f64, t41161: f64, t4537: f64, t4556: f64, t50887: f64, t50889: f64, t50891: f64, t50892: f64, t50894: f64, t50897: f64, t50898: f64) -> f64 {
    let t51792 = t890 * t2832;
    let t51802 = 6.0_f64 * t11064 * t1940 * t2408 * t4537 + 9.0_f64 * t14353 * t2403 * t2430 + 6.0_f64 * t14436 * t1940 * t51792 - 9.0_f64 * t2403 * t41161 * t4556 + t50887 - t50889 + t50891 + t50892 + t50894 + t50897 + t50898;
    t51802
}
