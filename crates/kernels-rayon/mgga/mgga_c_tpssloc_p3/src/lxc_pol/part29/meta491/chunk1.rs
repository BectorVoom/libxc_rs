//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1841/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1841(t1089: f64, t491: f64, t7327: f64, t15707: f64, t7376: f64, t24574: f64, t7365: f64, t1235: f64, t477: f64, t1090: f64, t7362: f64, t24837: f64, t3612: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24850 = t491 * t1089;
    let t24851 = t7327 * t24850;
    let t24852 = t15707 * t7376;
    let t24853 = t24851 * t24852;
    let t24856 = t24574 * t7365;
    let t24858 = t477 * t1235;
    let t24859 = t24858 * t1090;
    let t24860 = t7362 * t24859;
    let t24863 = t24837 * t3612;
    (t24850, t24851, t24852, t24853, t24856, t24858, t24859, t24860, t24863)
}
