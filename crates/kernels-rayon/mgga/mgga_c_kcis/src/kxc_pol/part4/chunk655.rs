//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 655/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk655(t174: f64, t236: f64, t3703: f64, t233: f64, t2645: f64, t447: f64, t637: f64, t446: f64, t1300: f64, t1640: f64, t1385: f64, t503: f64, t3187: f64, t3188: f64, t8: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t3704 = t236 * t3703;
    let t3705 = t233 * t3704;
    let t3706 = t3705 / 16.0_f64;
    let t3707 = piecewise3(t175, 0.0_f64, t2645);
    let t3708 = t447 * t3707;
    let t3709 = t3708 * t637;
    let t3710 = t446 * t3709;
    let t3711 = t3710 / 16.0_f64;
    let t3712 = t1300 * t1640;
    let t3713 = t446 * t3712;
    let t3714 = t3713 / 8.0_f64;
    let t3715 = t1385 * t1385;
    let t3716 = t503 * t503;
    let t3717 = 1.0_f64 / t3716;
    let t3718 = t3715 * t3717;
    let t3722 = t2645 * t8 - t3187 + t3188;
    (t3706, t3708, t3709, t3711, t3712, t3714, t3715, t3716, t3717, t3718, t3722)
}
