//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2894/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2894(t51913: f64, t51915: f64, t51890: f64, t51892: f64, t51894: f64, t51896: f64, t51899: f64, t51902: f64, t51907: f64, t51909: f64, t51911: f64, t51917: f64) -> f64 {
    let t52546 = 0.69463333333333333334e0_f64 * t51913;
    let t52547 = 0.11577222222222222222e0_f64 * t51915;
    let t52549 = -0.52945875e1_f64 * t51890 - 0.17648625e1_f64 * t51892 + 0.94674375e0_f64 * t51894 + 0.31558125e0_f64 * t51896 - 0.6618234375e1_f64 * t51899 + 0.2366859375e0_f64 * t51902 - 0.104195e0_f64 * t51907 - 0.83356000000000000001e0_f64 * t51909 + 0.13892666666666666667e0_f64 * t51911 + t52546 - t52547 - 0.41678000000000000001e0_f64 * t51917;
    t52549
}
