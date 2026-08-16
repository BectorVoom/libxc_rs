//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 840/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk840(t170: f64, t8892: f64, t596: f64, t7647: f64, t7650: f64, t7653: f64, t7656: f64, t7659: f64, t7661: f64, t7662: f64, t7664: f64, t7667: f64, t7669: f64, t7671: f64) -> f64 {
    let t8893 = t8892 * t170;
    let t8896 = 0.38527786510141256861e1_f64 * t7647 + 0.3429168e0_f64 * t7650 + t7653 + t7656 + t7659 + t7661 - 0.2077903092681775651e3_f64 * t7662 + 0.70178683471615754484e1_f64 * t7664 - 0.67745118933333333332e-2_f64 * t7667 + 0.14458108400402319789e-1_f64 * t7669 - 40.0_f64 * t7671 - 0.675260332e-1_f64 * t596 * t8893;
    t8896
}
