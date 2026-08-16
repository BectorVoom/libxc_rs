//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1176/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1176(t2121: f64, t24844: f64, t210: f64, t7371: f64, t7284: f64, t974: f64, t1089: f64, t491: f64, t7327: f64, t15707: f64, t7376: f64, t24574: f64, t7365: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24845 = t2121 * t24844;
    let t24847 = t7371 * t210;
    let t24848 = t974 * t7284;
    let t24849 = t24847 * t24848;
    let t24850 = t491 * t1089;
    let t24851 = t7327 * t24850;
    let t24852 = t15707 * t7376;
    let t24853 = t24851 * t24852;
    let t24856 = t24574 * t7365;
    (t24845, t24847, t24848, t24849, t24851, t24852, t24853, t24856)
}
