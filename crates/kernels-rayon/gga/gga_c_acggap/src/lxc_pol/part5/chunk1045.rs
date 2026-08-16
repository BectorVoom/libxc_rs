//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1045/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1045(t1328: f64, t3573: f64, t12930: f64, t1466: f64, t3409: f64, t4681: f64, t4685: f64, t4331: f64, t14173: f64, t4425: f64, t12816: f64, t13298: f64, t13299: f64, t525: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18097 = t3573 * t1328;
    let t18103 = t12930 * t1466;
    let t18105 = t3409 * t4681;
    let t18107 = t3409 * t4685;
    let t18109 = t3409 * t4331;
    let t18111 = t14173 * t4425;
    let t18119 = t13298 * t13299 * t525 * t12816;
    (t18097, t18103, t18105, t18107, t18109, t18111, t18119)
}
