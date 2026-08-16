//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1221/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1221(t39882: f64, t39886: f64, t39894: f64, t37937: f64, t37940: f64, t37947: f64, t37951: f64, t37954: f64, t37957: f64, t39874: f64, t39879: f64, t39891: f64) -> f64 {
    let t41600 = 0.45022119329691164871e0_f64 * t39882;
    let t41601 = 0.19514881078765566037e-1_f64 * t39886;
    let t41605 = 0.93149212406257582492e-1_f64 * t39894;
    let t41606 = 0.95219938395347901946e-2_f64 * t37937 + 0.5200933044032561138e0_f64 * t39874 + 0.28565981518604370584e-1_f64 * t37940 + 0.62295486109113302474e-1_f64 * t37947 + 0.18688645832733990742e0_f64 * t37951 + 0.43663693315433241794e-2_f64 * t39879 + t41600 - t41601 + 0.14282990759302185292e-1_f64 * t37954 + 0.47609969197673950973e-2_f64 * t37957 + 0.43663693315433241794e-2_f64 * t39891 - t41605;
    t41606
}
