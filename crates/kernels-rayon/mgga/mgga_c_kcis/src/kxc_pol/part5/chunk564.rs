//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 564/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk564(t3054: f64, t331: f64, t829: f64, t160: f64, t330: f64, t1071: f64, t740: f64, t1135: f64, t2861: f64, t1085: f64, t1094: f64, t1130: f64, t982: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3153 = 0.23911438650126355246e-1_f64 * t3054;
    let t3154 = t331 * t829;
    let t3158 = t160 * t330;
    let t3159 = 0.15538616723388920628e-3_f64 * t3158;
    let t3160 = t740 * t1071;
    let t3161 = t3160 * t829;
    let t3174 = t2861 * t1135;
    let t3177 = t1085 * t1094;
    let t3178 = t3177 * sigma0;
    let t3182 = t982 * t1130;
    (t3153, t3154, t3158, t3159, t3160, t3161, t3174, t3177, t3178, t3182)
}
