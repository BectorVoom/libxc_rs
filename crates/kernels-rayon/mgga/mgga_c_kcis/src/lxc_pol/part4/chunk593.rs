//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 593/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk593(t1079: f64, t2850: f64, t1056: f64, t2829: f64, t2845: f64, t113: f64, t2844: f64, t3054: f64, t331: f64, t829: f64, t160: f64, t330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3136 = t1079 * t2850;
    let t3139 = t1056 * t2850;
    let t3142 = t1079 * t2829;
    let t3145 = t1056 * t2845;
    let t3150 = t113 * t2844;
    let t3153 = 0.23911438650126355246e-1_f64 * t3054;
    let t3154 = t331 * t829;
    let t3158 = t160 * t330;
    (t3136, t3139, t3142, t3145, t3150, t3153, t3154, t3158)
}
