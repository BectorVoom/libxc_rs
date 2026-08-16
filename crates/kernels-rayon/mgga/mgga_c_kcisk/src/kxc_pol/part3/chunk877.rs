//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 877/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk877(t1175: f64, t3583: f64, t3539: f64, t1163: f64, t3587: f64, t1364: f64, t3544: f64, t13125: f64, t13131: f64, t13135: f64, t13140: f64, t13144: f64, t13150: f64, t13155: f64, t13158: f64, t1421: f64, t338: f64) -> f64 {
    let t13161 = t3583 * t1175;
    let t13162 = t3539 * t13161;
    let t13165 = t1163 * t3587;
    let t13166 = t3539 * t13165;
    let t13169 = t3583 * t1364;
    let t13170 = t3544 * t13169;
    let t13173 = -4.0_f64 * t338 * t13125 + 0.1478346675e-2_f64 * t1421 * t13131 - 0.59133867e-2_f64 * t1421 * t13135 + 0.39422577999999999999e-2_f64 * t1421 * t13140 + 0.39422577999999999999e-2_f64 * t1421 * t13144 + 0.49278222499999999999e-2_f64 * t1421 * t13150 - 0.32852148333333333333e-2_f64 * t1421 * t13155 + 0.32852148333333333333e-2_f64 * t1421 * t13158 + 0.295669335e-2_f64 * t1421 * t13162 + 0.295669335e-2_f64 * t1421 * t13166 - 0.19711289e-2_f64 * t1421 * t13170;
    t13173
}
