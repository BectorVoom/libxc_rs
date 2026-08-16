//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 999/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk999(t14190: f64, t14193: f64, t14195: f64, t14201: f64, t14206: f64, t14211: f64, t14216: f64, t14218: f64, t14220: f64, t14224: f64, t14226: f64, t14228: f64) -> f64 {
    let t14715 = 0.10446625e-1_f64 * t14190 + 0.27857666666666666666e-1_f64 * t14193 + 0.46429444444444444443e-2_f64 * t14195 + 0.18571777777777777778e-1_f64 * t14201 - 0.34822083333333333333e-2_f64 * t14206 + 0.51588271604938271604e-3_f64 * t14211 + 0.30952962962962962963e-2_f64 * t14216 + 0.23214722222222222222e-2_f64 * t14218 - 0.69644166666666666665e-2_f64 * t14220 + 0.11607361111111111111e-2_f64 * t14224 - 0.77382407407407407405e-3_f64 * t14226 - 0.12381185185185185185e-1_f64 * t14228;
    t14715
}
