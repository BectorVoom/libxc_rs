//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1467/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1467(t120995: f64, t120998: f64, t121003: f64, t121006: f64, t121009: f64, t121019: f64, t121132: f64, t121134: f64, t121136: f64, t121138: f64, t26969: f64, t32674: f64, t32676: f64, t33746: f64, t7171: f64, t8690: f64) -> f64 {
    let t124924 = 3.0_f64 * t26969 * t8690 + 3.0_f64 * t33746 * t7171 - t120995 - t120998 - t121003 - t121006 - t121009 - t121019 + t121132 - t121134 - t121136 - t121138 - t32674 - t32676;
    t124924
}
