//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 752/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk752(t174: f64, t4518: f64, t4521: f64, t740: f64, t833: f64, t44: f64, t4517: f64, t230: f64, t1655: f64, t908: f64, t1659: f64, t911: f64, t2633: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t4525 = piecewise3(t175, 0.0_f64, 4.0_f64 / 9.0_f64 * t4518 * t833 - 8.0_f64 / 3.0_f64 * t4521 * t740);
    let t4527 = (t4517 + t4525) * t44;
    let t4528 = t4527 * t230;
    let t4529 = t1655 * t908;
    let t4530 = t911 * t1659;
    let t4532 = 2.0_f64 * t2633;
    (t4527, t4528, t4529, t4530, t4532)
}
