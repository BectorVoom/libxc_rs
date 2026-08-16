//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1377/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1377(t42213: f64, t47787: f64, t76587: f64, t76595: f64, t76610: f64, t76618: f64, t76626: f64, t76899: f64, t76903: f64, t76906: f64, t76912: f64, t77102: f64, t77105: f64, t77107: f64) -> f64 {
    let t77301 = t42213 - 0.27785333333333333334e0_f64 * t76899 + 0.83356e0_f64 * t76903 - 0.13892666666666666667e0_f64 * t76906 - 0.375102e1_f64 * t76912 + 0.3529725e1_f64 * t77102 + 0.21424148148148148148e1_f64 * t47787 - 0.52945875e1_f64 * t77105 + 0.2366859375e0_f64 * t77107 - 0.34431666666666666667e1_f64 * t76587 + 0.123954e2_f64 * t76595 - 0.13772666666666666667e1_f64 * t76610 - 0.185931e2_f64 * t76618 + 0.41318e1_f64 * t76626;
    t77301
}
