//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 851/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk851(t5871: f64, t5878: f64, t7026: f64, t7027: f64, t170: f64, t3129: f64, t584: f64, t591: f64, t159: f64, t5774: f64, t5777: f64, t5793: f64, t5919: f64, t5920: f64, t5923: f64, t7825: f64, t7827: f64, t7831: f64, t7832: f64) -> (f64, f64) {
    let t9005 = -t5871 - t7026 + t7027 + t5878;
    let t9006 = t9005 * t170;
    let t9010 = t584 * t3129 * t591;
    let t9012 = -t5774 + t5919 - 0.10005107252466666667e-2_f64 * t5920 - t5777 - 0.64212977516902094771e0_f64 * t7825 + 0.43374325201206959368e-1_f64 * t7827 - t5793 - t7831 + 0.53360572013155555555e-2_f64 * t7832 + 0.285764e-1_f64 * t159 * t9006 + t5923 - 0.571528e-1_f64 * t9010;
    (t9005, t9012)
}
