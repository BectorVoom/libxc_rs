//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 882/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk882(t1873: f64, t28713: f64, t1869: f64, t23038: f64, t2441: f64, t1800: f64, t6719: f64, t8878: f64, t6965: f64, t8786: f64, t1693: f64, t23320: f64, t23872: f64, t28327: f64, t28329: f64, t28334: f64, t28703: f64, t28706: f64, t28711: f64) -> (f64, f64, f64, f64, f64) {
    let t28714 = t1873 * t28713;
    let t28715 = t1869 * t28714;
    let t28717 = t23038 * t2441;
    let t28718 = t1800 * t28717;
    let t28719 = t1869 * t28718;
    let t28721 = t6719 * t8878;
    let t28722 = t1869 * t28721;
    let t28724 = t6965 * t8786;
    let t28725 = t1800 * t28724;
    let t28726 = t1869 * t28725;
    let t28728 = -0.49745833333333333332e-2_f64 * t28327 + 0.579e0_f64 * t1693 * t28329 + 0.19898333333333333333e-1_f64 * t28334 + 0.24872916666666666666e-2_f64 * t28703 - 0.19898333333333333333e-1_f64 * t28706 + 0.49745833333333333332e-2_f64 * t23320 - 0.66327777777777777776e-2_f64 * t23872 - 0.16581944444444444444e-2_f64 * t28711 - 0.72960555555555555553e-1_f64 * t28715 + 0.48640370370370370369e-1_f64 * t28719 - 0.2653111111111111111e-1_f64 * t28722 - 0.13265555555555555555e-1_f64 * t28726;
    (t28715, t28719, t28722, t28726, t28728)
}
