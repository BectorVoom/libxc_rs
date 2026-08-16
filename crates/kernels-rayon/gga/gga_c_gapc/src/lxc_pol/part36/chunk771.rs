//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 771/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk771(t649: f64, t9197: f64, t197: f64, t5479: f64, t2986: f64, t1018: f64, t1875: f64, t1877: f64, t3096: f64, t2990: f64, t3088: f64, t5803: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9198 = t9197 * t649;
    let t9200 = t197 * t5479;
    let t9201 = t2986 * t9200;
    let t9203 = t1875 * t1018;
    let t9204 = t3096 * t1877;
    let t9205 = t9203 * t9204;
    let t9207 = t3088 * t2990;
    let t9209 = t197 * t5803;
    (t9198, t9201, t9203, t9205, t9207, t9209)
}
