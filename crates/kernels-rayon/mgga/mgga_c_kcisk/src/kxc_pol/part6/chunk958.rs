//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 958/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk958(t23286: f64, t23320: f64, t23872: f64, t28320: f64, t28327: f64, t28334: f64, t28703: f64, t28706: f64, t28711: f64, t28715: f64, t28719: f64, t28722: f64) -> f64 {
    let t29971 = -0.52233124999999999998e-2_f64 * t28320 - 0.46429444444444444443e-2_f64 * t23286 - 0.34822083333333333333e-2_f64 * t28327 + 0.13928833333333333333e-1_f64 * t28334 + 0.17411041666666666666e-2_f64 * t28703 - 0.13928833333333333333e-1_f64 * t28706 + 0.34822083333333333333e-2_f64 * t23320 - 0.46429444444444444443e-2_f64 * t23872 - 0.11607361111111111111e-2_f64 * t28711 - 0.51072388888888888887e-1_f64 * t28715 + 0.34048259259259259259e-1_f64 * t28719 - 0.18571777777777777778e-1_f64 * t28722;
    t29971
}
