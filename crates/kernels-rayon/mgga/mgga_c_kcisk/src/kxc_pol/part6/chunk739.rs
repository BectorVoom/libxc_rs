//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 739/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk739(t12884: f64, t325: f64, t4459: f64, t512: f64, t507: f64, t12998: f64, t12974: f64, t1527: f64, t4462: f64, t515: f64, t1588: f64, t3532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14743 = t325 * t12884;
    let t14756 = 1.0_f64 / t4459 / t512;
    let t14757 = t507 * t14756;
    let t14784 = 0.46308888888888888888e0_f64 * t12998;
    let t14785 = 0.16068111111111111111e1_f64 * t12974;
    let t14797 = 1.0_f64 / t4459 / t1527;
    let t14798 = t507 * t14797;
    let t14800 = 1.0_f64 / t4462 / t515;
    let t14831 = 0.53272592592592592592e-1_f64 * t12974;
    let t14909 = t1588 * t3532;
    (t14743, t14757, t14784, t14785, t14798, t14800, t14831, t14909)
}
