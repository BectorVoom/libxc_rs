//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 939/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk939(t2751: f64, t5734: f64, t1916: f64, t1938: f64, t1955: f64, t2834: f64, t2853: f64, t5830: f64, t5835: f64, t5838: f64, t5871: f64, t5903: f64, t7231: f64, t7234: f64, t7237: f64, t7241: f64, t7244: f64, t7248: f64, t7255: f64, t7258: f64, t7261: f64) -> (f64, f64) {
    let t7265 = 4.0_f64 * t5734 * t2751;
    let t7266 = -4.0_f64 * t1916 * t7231 - 2.0_f64 * t1916 * t7234 - 0.19298375398431042081e3_f64 * t5830 * t7237 + 0.64327917994770140268e2_f64 * t1938 * t7241 + 0.32163958997385070134e2_f64 * t1938 * t7244 + 0.2069040516770936012e4_f64 * t5871 * t7248 - 0.23392894490538584828e1_f64 * t5903 * t2834 + 0.34631718211362927518e2_f64 * t5835 * t2853 - 0.23392894490538584828e1_f64 * t1955 * t7255 - 0.11696447245269292414e1_f64 * t1955 * t7258 - 0.10389515463408878255e3_f64 * t5838 * t7261 + t7265;
    (t7265, t7266)
}
