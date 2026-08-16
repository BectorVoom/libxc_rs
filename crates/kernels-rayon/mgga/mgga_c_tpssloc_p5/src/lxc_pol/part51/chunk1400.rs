//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1400/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1400(t101551: f64, t114865: f64, t114882: f64, t114892: f64, t118847: f64, t118850: f64, t118851: f64, t121511: f64, t121531: f64, t121552: f64, t121614: f64, t1912: f64, t24297: f64, t25168: f64, t25330: f64, t26680: f64, t2713: f64, t33399: f64, t6627: f64, t6631: f64, t7087: f64, t7538: f64, t855: f64, t858: f64, t92386: f64) -> f64 {
    let t121623 = -t6627 * t26680 - t7087 * t25330 - 6.0_f64 * t25168 * t101551 * t6631 + t118847 - t855 * t858 * (t121511 + t121531 + t121552 + t121614) - t118850 - t114865 - t24297 * t7538 - t2713 * t33399 + 0.19190897446562641759e-1_f64 * t114882 - t118851 + t114892 - t92386 * t1912;
    t121623
}
