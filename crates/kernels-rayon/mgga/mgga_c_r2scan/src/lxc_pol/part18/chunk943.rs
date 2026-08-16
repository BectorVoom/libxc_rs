//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 943/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk943(t10935: f64, t3446: f64, t766: f64, t2279: f64, t3428: f64, t3430: f64, t10810: f64, t1104: f64, t3429: f64, t158: f64, t607: f64, t122: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10937 = t3446 * t10935 * t766;
    let t10943 = t2279 * t3428;
    let t10944 = t10943 * t3430;
    let t10945 = 0.30487649791575028314e-3_f64 * t10944;
    let t10946 = t10810 * t1104;
    let t10947 = t3429 * t10946;
    let t10948 = 0.81300399444200075504e-3_f64 * t10947;
    let t10949 = t158 * t607;
    let t10950 = t10949 * t122;
    (t10937, t10943, t10945, t10946, t10948, t10949, t10950)
}
