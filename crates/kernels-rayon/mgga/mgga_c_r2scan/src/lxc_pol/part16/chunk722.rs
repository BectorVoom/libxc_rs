//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 722/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk722(t5720: f64, t61: f64, t1871: f64, t584: f64, t608: f64, t4741: f64, t5309: f64, t5312: f64, t5315: f64, t171: f64, t718: f64, t226: f64, t5456: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5853 = 0.65061487801810439052e-1_f64 * t61 * t5720;
    let t5855 = t584 * t608 * t1871;
    let t5860 = 0.32547666666666666667e-1_f64 * t4741;
    let t5861 = -0.14816666666666666667e-1_f64 * t5309 + 0.9877777777777777778e-2_f64 * t5312 - 0.46096296296296296297e-1_f64 * t5315 - t5860;
    let t5864 = 0.571528e-1_f64 * t584 * t171 * t5861;
    let t5865 = t61 * t718;
    let t5866 = t226 * t5456;
    (t5853, t5855, t5860, t5864, t5865, t5866)
}
