//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 843/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk843(t3128: f64, t585: f64, t159: f64, t617: f64, t5331: f64, t5335: f64, t5338: f64, t5340: f64, t5344: f64, t5346: f64, t5350: f64, t5354: f64, t5355: f64, t7708: f64) -> f64 {
    let t8915 = t3128 * t585;
    let t8916 = t159 * t8915;
    let t8917 = t8916 * t617;
    let t8925 = 0.84681398666666666666e-3_f64 * t8917 + 16.0_f64 * t7708 - t5331 + t5335 - 0.23392894490538584828e1_f64 * t5338 + 0.34631718211362927518e2_f64 * t5340 + 0.35089341735807877242e1_f64 * t5344 - 0.10389515463408878255e3_f64 * t5346 - t5350 - t5354 - 0.11696447245269292414e1_f64 * t5355;
    t8925
}
