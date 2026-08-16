//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 848/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk848(t1416: f64, t959: f64, t5237: f64, t7647: f64, t7650: f64, t7653: f64, t7656: f64, t7659: f64, t7661: f64, t7662: f64, t7664: f64, t7667: f64, t7669: f64) -> f64 {
    let t7671 = t1416 * t959;
    let t7673 = -0.21687162600603479684e-1_f64 * t5237 + 0.19263893255070628431e1_f64 * t7647 + 0.1714584e0_f64 * t7650 - t7653 + t7656 + t7659 + t7661 - 0.10389515463408878255e3_f64 * t7662 + 0.35089341735807877242e1_f64 * t7664 - 0.33872559466666666666e-2_f64 * t7667 + 0.72290542002011598948e-2_f64 * t7669 - 20.0_f64 * t7671;
    t7673
}
