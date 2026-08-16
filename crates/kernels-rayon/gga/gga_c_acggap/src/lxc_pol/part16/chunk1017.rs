//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1017/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1017(t35500: f64, t7380: f64, t34050: f64, t2095: f64, t33901: f64, t33884: f64, t1998: f64, t4503: f64, t5124: f64, t7647: f64, t7310: f64, t8878: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35909 = t7380 * t35500;
    let t35910 = 0.4584375e-1_f64 * t35909;
    let t35911 = t7380 * t34050;
    let t35912 = 0.4584375e-1_f64 * t35911;
    let t35913 = t2095 * t33901;
    let t35914 = 0.305625e-1_f64 * t35913;
    let t35915 = t2095 * t33884;
    let t35916 = 0.305625e-1_f64 * t35915;
    let t35917 = t1998 * t4503;
    let t35918 = 0.17149607247227894789e-2_f64 * t35917;
    let t35919 = t7647 * t5124;
    let t35920 = 0.17149607247227894789e-2_f64 * t35919;
    let t35924 = t7310 * t8878;
    (t35910, t35912, t35914, t35916, t35918, t35920, t35924)
}
