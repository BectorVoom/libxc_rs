//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1160/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1160(t31629: f64, t31646: f64, t1426: f64, t429: f64, t598: f64, t8539: f64, t35500: f64, t7380: f64, t34050: f64, t2095: f64, t33901: f64, t33884: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35898 = 0.12862205435420921092e-1_f64 * t31629;
    let t35904 = 0.32012600194825403606e-1_f64 * t31646;
    let t35907 = t598 * t1426 * t429 * t8539;
    let t35909 = t7380 * t35500;
    let t35910 = 0.4584375e-1_f64 * t35909;
    let t35911 = t7380 * t34050;
    let t35912 = 0.4584375e-1_f64 * t35911;
    let t35913 = t2095 * t33901;
    let t35914 = 0.305625e-1_f64 * t35913;
    let t35915 = t2095 * t33884;
    (t35898, t35904, t35907, t35910, t35912, t35914, t35915)
}
