//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 885/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk885(t13277: f64, t1433: f64, t457: f64, t12992: f64, t1428: f64, t13213: f64, t13217: f64, t13223: f64, t13228: f64, t13231: f64, t13235: f64, t13238: f64, t13240: f64, t13242: f64, t1421: f64, t456: f64) -> (f64, f64, f64) {
    let t13278 = t1433 * t13277;
    let t13279 = t457 * t13278;
    let t13282 = t1428 * t12992;
    let t13283 = t457 * t13282;
    let t13286 = -0.4435040025e-2_f64 * t1421 * t13213 - 0.4435040025e-2_f64 * t1421 * t13217 + 0.29201909629629629629e-2_f64 * t1421 * t13223 + 0.887008005e-2_f64 * t1421 * t13228 - 0.26281718666666666667e-2_f64 * t13231 - 0.65704296666666666666e-2_f64 * t1421 * t13235 - 0.43802864444444444445e-3_f64 * t13238 + 0.13140859333333333334e-2_f64 * t13240 + 0.21901432222222222222e-2_f64 * t13242 - 0.98556445e-3_f64 * t456 * t13279 + 0.1478346675e-2_f64 * t456 * t13283;
    (t13278, t13282, t13286)
}
