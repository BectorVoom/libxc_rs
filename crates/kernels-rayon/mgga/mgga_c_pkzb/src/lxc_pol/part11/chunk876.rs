//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 876/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk876(t1108: f64, t2848: f64, t3608: f64, t721: f64, t3605: f64, t1916: f64, t1938: f64, t1955: f64, t1977: f64, t2834: f64, t2853: f64, t5830: f64, t5838: f64, t5871: f64, t7315: f64, t7494: f64, t9410: f64, t9413: f64, t9416: f64, t9419: f64, t9423: f64, t9426: f64, t9430: f64, t9437: f64) -> (f64, f64, f64, f64) {
    let t9440 = t1108 * t2848;
    let t9443 = t3608 * t721;
    let t9446 = t3605 * t721;
    let t9449 = 6.0_f64 * t1938 * t9410 - 4.0_f64 * t1916 * t9413 - 0.19298375398431042081e3_f64 * t5830 * t9416 - 2.0_f64 * t1916 * t9419 + 0.32163958997385070134e2_f64 * t1938 * t9423 + 0.64327917994770140268e2_f64 * t1938 * t9426 + 0.2069040516770936012e4_f64 * t5871 * t9430 - 0.23392894490538584828e1_f64 * t7494 * t2834 + 0.34631718211362927517e2_f64 * t7315 * t2853 + 0.35089341735807877242e1_f64 * t1977 * t9437 - 0.23392894490538584828e1_f64 * t1955 * t9440 - 0.10389515463408878255e3_f64 * t5838 * t9443 - 0.11696447245269292414e1_f64 * t1955 * t9446;
    (t9440, t9443, t9446, t9449)
}
