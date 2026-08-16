//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2806/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2806(t51564: f64, t2722: f64, t50474: f64, t2782: f64, t39597: f64, t14586: f64, t10529: f64, t10115: f64, t1576: f64, t14593: f64, t2470: f64, t874: f64) -> (f64, f64, f64, f64, f64) {
    let t51565 = 0.34697458558045176417e-2_f64 * t51564;
    let t51570 = t50474 * t2722;
    let t51572 = t2782 * t39597 * t51570;
    let t51574 = t14586 * t2722;
    let t51576 = t2782 * t10529 * t51574;
    let t51578 = t10115 * t1576;
    let t51587 = t874 * t14593 * t2470;
    (t51565, t51572, t51576, t51578, t51587)
}
