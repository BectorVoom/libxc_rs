//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 705/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk705(t2850: f64, t2880: f64, t298: f64, t142: f64, t2884: f64, t2888: f64, t2917: f64, t840: f64, t2920: f64, t55: f64, t12535: f64, t2879: f64, t2885: f64, t2887: f64, t829: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12604 = 0.53425e-1_f64 * t298 * t2850 * t2880;
    let t12605 = t142 * t2884;
    let t12608 = 0.85917146441092277512e0_f64 * t298 * t12605 * t2888;
    let t12610 = 1.0_f64 / t2917 / t840;
    let t12613 = 1.0_f64 / t2920 / t55;
    let t12614 = t12610 * t12535 * t12613;
    let t12620 = 0.48245472966453314466e2_f64 * t2885 * t2879 * t2887 * t829;
    (t12604, t12608, t12610, t12613, t12614, t12620)
}
