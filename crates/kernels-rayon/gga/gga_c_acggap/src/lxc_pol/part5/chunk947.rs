//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 947/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk947(t1390: f64, t712: f64, t1388: f64, t1381: f64, t2987: f64, t2868: f64, t484: f64, t2970: f64, t1268: f64, t495: f64, t2981: f64, t715: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14959 = t712 * t1390;
    let t14965 = t712 * t1388;
    let t14967 = t1381 * t2987;
    let t14969 = t2868 * t484;
    let t14972 = t2970 * t484;
    let t14974 = t495 * t1268;
    let t14984 = t1381 * t2981;
    let t14986 = t715 * t1390;
    (t14959, t14965, t14967, t14969, t14972, t14974, t14984, t14986)
}
