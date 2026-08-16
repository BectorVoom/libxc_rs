//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1408/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1408(t11064: f64, t11075: f64, t1940: f64, t2394: f64, t2408: f64, t2832: f64, t39760: f64, t39764: f64, t39767: f64, t39770: f64, t39773: f64, t39775: f64, t39778: f64, t39780: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t4541: f64) -> f64 {
    let t41150 = 12.0_f64 * t11064 * t1940 * t2408 * t2832 + 36.0_f64 * t11075 * t2394 * t4541 + t39760 - t39764 + t39767 + t39770 + t39773 - t39775 + t39778 + t39780 - t39783 - t39786 - t39791 - t39795;
    t41150
}
