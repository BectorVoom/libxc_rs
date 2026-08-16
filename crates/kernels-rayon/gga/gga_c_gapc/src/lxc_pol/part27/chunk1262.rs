//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1262/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1262(t11210: f64, t21054: f64, t25076: f64, t8286: f64, t11208: f64, t19916: f64, t11185: f64, t2953: f64, t11249: f64, t8352: f64, t1928: f64, t2941: f64, t640: f64) -> (f64, f64, f64, f64, f64) {
    let t35552 = t8286 * t25076 * t11210 * t21054;
    let t35555 = t11208 * t11210 * t19916;
    let t35557 = t2953 * t11185;
    let t35559 = t8352 * t11249;
    let t35562 = t2941 * t640 * t1928;
    (t35552, t35555, t35557, t35559, t35562)
}
