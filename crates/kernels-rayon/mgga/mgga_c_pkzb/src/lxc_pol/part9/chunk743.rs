//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 743/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk743(t12: f64, t24: f64, t1692: f64, t192: f64, t1646: f64, t1837: f64, t207: f64, t5094: f64, t5100: f64, t653: f64, t1655: f64, t2179: f64, t333: f64, t5107: f64, t5113: f64, t822: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t5196 = t192 * t1692;
    let t5207 = piecewise3(t84, 0.0_f64, 8.0_f64 / 27.0_f64 * t1837 * t5094 - 2.0_f64 / 3.0_f64 * t653 * t1646 + 2.0_f64 / 3.0_f64 * t207 * t5100);
    let t5215 = piecewise3(t90, 0.0_f64, 8.0_f64 / 27.0_f64 * t2179 * t5107 - 2.0_f64 / 3.0_f64 * t822 * t1655 + 2.0_f64 / 3.0_f64 * t333 * t5113);
    (t5196, t5207, t5215)
}
