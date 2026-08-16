//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1048/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1048(t11436: f64, t11440: f64, t11443: f64, t11459: f64, t11466: f64, t11469: f64, t11471: f64, t11475: f64, t11477: f64, t11481: f64, t11486: f64, t11490: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12099 = 0.21720231316129303386e-4_f64 * t11436;
    let t12100 = 0.21720231316129303386e-4_f64 * t11440;
    let t12101 = 0.5686343261418565457e-6_f64 * t11443;
    let t12104 = 0.10110318318802209383e-5_f64 * t11459;
    let t12107 = 0.31675337336021900771e-5_f64 * t11466;
    let t12108 = 0.67530371184977617164e-6_f64 * t11469;
    let t12109 = 0.67530371184977617164e-6_f64 * t11471;
    let t12110 = 0.40022999988963401107e-7_f64 * t11475;
    let t12111 = 0.40096157891080460192e-6_f64 * t11477;
    let t12112 = 0.16908181191593721013e-5_f64 * t11481;
    let t12113 = 0.2318836277704281739e-4_f64 * t11486;
    let t12114 = 0.4637672555408563478e-4_f64 * t11490;
    (t12099, t12100, t12101, t12104, t12107, t12108, t12109, t12110, t12111, t12112, t12113, t12114)
}
