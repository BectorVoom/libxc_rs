//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1086/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1086(t1010: f64, t5843: f64, t5378: f64, t5381: f64, t12884: f64, t247: f64, t6421: f64, t1261: f64, t1785: f64, t5390: f64, t5357: f64, t5373: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21213 = t5843 * t1010;
    let t21216 = t5381 * t5378;
    let t21233 = t247 * t12884 * t6421;
    let t21234 = t1261 * t21233;
    let t21242 = t1785 * t5390;
    let t21249 = t5373 * t5357;
    (t21213, t21216, t21233, t21234, t21242, t21249)
}
