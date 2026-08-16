//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2820/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2820(t10489: f64, t11054: f64, t11084: f64, t1940: f64, t198: f64, t207: f64, t2403: f64, t39989: f64, t4343: f64, t4541: f64, t4542: f64, t4556: f64, t50106: f64, t50114: f64, t50115: f64, t50151: f64, t50190: f64, t50216: f64, t50250: f64, t50276: f64, t50853: f64, t50857: f64, t51218: f64, t51253: f64, t51723: f64, t51762: f64, t765: f64, t892: f64) -> f64 {
    let t51769 = -9.0_f64 * t2403 * t11084 * t4343 + t50106 - t39989 + 6.0_f64 * t4541 * t4542 * t10489 - t1940 * t4556 * t11054 + t50114 + t50115 + 3.0_f64 * t198 * t765 * t50151 + t198 * t207 * (t50190 + t50216 + t50250 + t50276 + t51218 + t51253 + t51723 + t51762) * t892 - t50853 - t50857;
    t51769
}
