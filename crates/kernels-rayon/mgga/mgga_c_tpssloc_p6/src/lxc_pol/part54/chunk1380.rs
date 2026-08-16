//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1380/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1380(t114790: f64, t23164: f64, t7479: f64, t114866: f64, t1880: f64, t7488: f64, t23168: f64, t33419: f64, t112863: f64, t114785: f64, t114827: f64, t118828: f64, t118831: f64, t118837: f64, t118838: f64, t118841: f64, t1528: f64, t23281: f64, t24305: f64, t25168: f64, t25183: f64, t26728: f64, t7517: f64, t7842: f64) -> f64 {
    let t121464 = t23164 * t114790 * t7479;
    let t121467 = t1880 * t114866 * t7488;
    let t121469 = t23168 * t33419;
    let t121479 = 0.82246703342411321825e-2_f64 * t121464 - 0.82246703342411321825e-2_f64 * t121467 + 0.38381794893125283518e-1_f64 * t121469 - 0.41123351671205660912e-2_f64 * t114827 + t118828 - 6.0_f64 * t25168 * t26728 * t25183 + 2.0_f64 * t24305 * t7517 + t118831 + t112863 - t118837 - t23281 * t7842 - t118838 - t118841 - t114785 * t1528;
    t121479
}
