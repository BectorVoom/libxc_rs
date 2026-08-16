//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 991/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk991(t13969: f64, t4584: f64, t1041: f64, t4589: f64, t2960: f64, t4603: f64, t1606: f64, t698: f64, t973: f64, t1043: f64, t2770: f64, t10277: f64, t3061: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14134 = t13969 * t4584;
    let t14136 = t1041 * t14134 / 1728.0_f64;
    let t14137 = t13969 * t4589;
    let t14139 = 5.0_f64 / 10368.0_f64 * t1041 * t14137;
    let t14158 = t2960 * t4603 / 162.0_f64;
    let t14159 = t698 * t1606;
    let t14160 = t973 * t14159;
    let t14164 = t1043 * t2770;
    let t14172 = t3061 * t10277;
    (t14136, t14139, t14158, t14160, t14164, t14172)
}
