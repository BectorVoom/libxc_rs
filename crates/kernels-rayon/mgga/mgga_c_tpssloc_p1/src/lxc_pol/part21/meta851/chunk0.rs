//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3079/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3079(t3271: f64, t43889: f64, t5992: f64, t11243: f64, t5999: f64, t43880: f64, t11265: f64, t63323: f64, t63327: f64, t63330: f64, t63848: f64, t63853: f64, t63856: f64, t63858: f64, t63860: f64, t63862: f64, t63865: f64, t63867: f64) -> (f64, f64, f64, f64, f64) {
    let t63870 = t43889 * t5992 * t3271;
    let t63873 = t11243 * t5999 * t3271;
    let t63876 = t43880 * t5992 * t3271;
    let t63879 = t11265 * t5999 * t3271;
    let t63881 = -0.76790625e-1_f64 * t63848 + 0.13287407407407407407e1_f64 * t63323 + 0.71752000000000000001e1_f64 * t63327 - 0.47834666666666666668e1_f64 * t63330 + 0.3071625e0_f64 * t63853 + 0.3071625e0_f64 * t63856 + 0.15358125e0_f64 * t63858 + 0.142419375e1_f64 * t63860 - 0.1898925e1_f64 * t63862 - 0.1898925e1_f64 * t63865 - 0.9494625e0_f64 * t63867 + 0.1151859375e0_f64 * t63870 - 0.76790625e-1_f64 * t63873 - 0.3560484375e1_f64 * t63876 + 0.142419375e1_f64 * t63879;
    (t63870, t63873, t63876, t63879, t63881)
}
