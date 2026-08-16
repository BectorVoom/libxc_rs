//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1239/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1239(t10829: f64, t1979: f64, t10830: f64, t10834: f64, t17638: f64, t1955: f64, t1977: f64, t21212: f64, t2848: f64, t30236: f64, t30238: f64, t30242: f64, t30245: f64, t30248: f64, t30252: f64, t3608: f64, t5838: f64, t5845: f64, t721: f64, t7315: f64, t7494: f64, t9401: f64, t9402: f64, t9446: f64, t9452: f64, t9455: f64) -> f64 {
    let t30459 = t10829 * t1979;
    let t30466 = -0.35089341735807877242e1_f64 * t7494 * t9446 + 0.51947577317044391276e2_f64 * t7315 * t9452 + 0.10389515463408878255e3_f64 * t7315 * t9455 + 0.30762056574649219972e4_f64 * t21212 * t9402 - 0.31168546390226634765e3_f64 * t5838 * t3608 * t2848 - 0.12304822629859687989e5_f64 * t17638 * t10834 * t721 - 0.11696447245269292414e1_f64 * t1955 * t10830 * t721 + 0.17315859105681463759e2_f64 * t1977 * t30459 * t721 + 0.30762056574649219974e4_f64 * t5845 * t9401 * t2848 - t30236 - t30238 + t30242 + t30245 + t30248 - t30252;
    t30466
}
