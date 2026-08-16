//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1108/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1108(t1160: f64, t4162: f64, t6482: f64, t3077: f64, t6535: f64, t4210: f64, t6461: f64, t1539: f64, t1907: f64, t406: f64, t377: f64, t6510: f64) -> (f64, f64, f64, f64, f64) {
    let t19862 = t1160 * t6482 * t4162;
    let t19864 = t3077 * t6535;
    let t19870 = t1160 * t6461 * t4210;
    let t19874 = t1160 * t1907 * t406 * t1539;
    let t19880 = t377 * t6510;
    (t19862, t19864, t19870, t19874, t19880)
}
