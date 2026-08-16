//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1372/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1372(t41962: f64, t47787: f64, t76587: f64, t76595: f64, t76610: f64, t76618: f64, t76626: f64, t76899: f64, t76903: f64, t76906: f64, t76912: f64, t77102: f64, t77105: f64, t77107: f64) -> f64 {
    let t77218 = t41962 - 0.22076e0_f64 * t76899 + 0.66228e0_f64 * t76903 - 0.11038e0_f64 * t76906 - 0.298026e1_f64 * t76912 + 0.258925e1_f64 * t77102 + 0.12524296296296296297e1_f64 * t47787 - 0.3883875e1_f64 * t77105 + 0.6189328125e-1_f64 * t77107 - 0.20128333333333333334e1_f64 * t76587 + 0.72462e1_f64 * t76595 - 0.80513333333333333332e0_f64 * t76610 - 0.108693e2_f64 * t76618 + 0.24154e1_f64 * t76626;
    t77218
}
