//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2338/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2338(t10236: f64, t14165: f64, t13831: f64, t13847: f64, t2986: f64, t10913: f64, t4337: f64, t10254: f64, t12648: f64, t43070: f64, t10190: f64, t13835: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47887 = t10236 * t14165;
    let t47907 = t2986 * t13847 * t13831;
    let t47915 = t4337 * t10913;
    let t47919 = t10254 * t12648;
    let t47927 = t43070 * t14165;
    let t47938 = t2986 * t10190 * t13835;
    (t47887, t47907, t47915, t47919, t47927, t47938)
}
