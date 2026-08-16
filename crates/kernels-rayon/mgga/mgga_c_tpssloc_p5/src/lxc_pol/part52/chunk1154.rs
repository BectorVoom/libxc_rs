//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1154/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1154(t30728: f64, t858: f64, t798: f64, t8347: f64, t225: f64, t8348: f64, t218: f64, t30725: f64, t6547: f64, t8336: f64, t1912: f64, t23278: f64, t23281: f64, t259: f64, t2597: f64, t2713: f64, t30673: f64, t6627: f64, t6663: f64, t8353: f64, t8363: f64, t855: f64, t866: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30729 = t858 * t30728;
    let t30732 = t798 * t8347;
    let t30741 = t8348 * t225;
    let t30745 = t218 * t30725;
    let t30748 = 0.38381794893125283518e-1_f64 * t6547 * t8336;
    let t30751 = -2.0_f64 * t1912 * t23278 - 2.0_f64 * t1912 * t23281 + t259 * t30732 + t259 * t30745 + 2.0_f64 * t2597 * t8353 - t2597 * t8363 + 2.0_f64 * t2713 * t8353 - t2713 * t8363 - t30729 * t855 - t30741 * t866 - 2.0_f64 * t6627 * t6663 - t30673 + t30748;
    (t30729, t30732, t30741, t30745, t30748, t30751)
}
