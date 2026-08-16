//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1272/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1272<F: Float>(t2904: F, t41733: F, t951: F, t959: F, t41654: F, t41642: F, t41646: F, t41651: F, t41656: F, t41658: F, t41660: F, t41662: F, t41669: F, t41673: F, t41675: F) -> (F, F) {
    let t41737 = F::cast_from(0.35089341735807877242e1_f64) * t959 * t2904 * t41733 * t951;
    let t41741 = F::cast_from(0.96141975308641975307e-1_f64) * t41654;
    let t41749 = F::cast_from(0.55625000000000000001e-1_f64) * t41642 + F::cast_from(0.74166666666666666668e-1_f64) * t41646 + F::cast_from(0.22249999999999999999e0_f64) * t41651 + t41741 - F::cast_from(0.24722222222222222222e-1_f64) * t41656 - F::cast_from(0.16481481481481481482e-1_f64) * t41658 + F::cast_from(0.13734567901234567901e-1_f64) * t41660 + F::cast_from(0.12361111111111111111e-1_f64) * t41662 - F::cast_from(0.27469135802469135803e-1_f64) * t41669 - F::cast_from(0.92708333333333333333e-2_f64) * t41673 + F::cast_from(0.49444444444444444445e-1_f64) * t41675;
    (t41737, t41749)
}
