//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1003/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1003(t4987: f64, t7647: f64, t1980: f64, t34487: f64, t7476: f64, t31126: f64, t31128: f64, t2314: f64, t31258: f64, t31140: f64, t1982: f64, t568: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35317 = t7647 * t4987;
    let t35348 = t1980 * t7476 * t34487;
    let t35352 = 0.1324375e0_f64 * t31126;
    let t35353 = 0.57165357490759649296e-3_f64 * t31128;
    let t35359 = t31258 * t2314;
    let t35361 = 0.1528125e-1_f64 * t31140;
    let t35364 = t568 * t1982;
    (t35317, t35348, t35352, t35353, t35359, t35361, t35364)
}
