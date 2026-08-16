//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2298/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2298(t6238: f64, t7284: f64, t24574: f64, t29546: f64, t103314: f64, t1090: f64, t11605: f64, t1238: f64, t1251: f64, t1761: f64, t24589: f64, t24601: f64, t24893: f64, t27382: f64, t27406: f64, t27742: f64, t27784: f64, t27792: f64, t27821: f64, t27826: f64, t27830: f64, t29794: f64, t3598: f64, t4930: f64, t4945: f64, t5059: f64, t5060: f64, t5089: f64, t6244: f64, t7283: f64, t7287: f64, t8087: f64, t94395: f64, t94648: f64, t94656: f64) -> f64 {
    let t103391 = t7284 * t6238;
    let t103413 = t24574 * t29546;
    let t103415 = -2.0_f64 * t27830 * t5089 + 2.0_f64 * t24893 * t6244 - 12.0_f64 * t27784 * t11605 * t8087 * t5059 + t94648 + 0.16449340668482264365e-1_f64 * t7283 * t4930 * t27382 - 2.0_f64 * t94656 * t1761 - 0.27415567780803773942e-2_f64 * t7283 * t103391 * t7287 + 2.0_f64 * t1238 * t3598 * t29794 * t1251 + 4.0_f64 * t27792 * t5060 - 2.0_f64 * t4945 * t27742 + 0.43864908449286038306e-1_f64 * t27406 * t27826 + 0.27415567780803773942e-2_f64 * t24589 * t24601 * t103314 * t1090 - 0.14621636149762012769e-1_f64 * t94395 * t27821 - 2.0_f64 * t27792 * t5089 - 0.27415567780803773942e-2_f64 * t103413;
    t103415
}
