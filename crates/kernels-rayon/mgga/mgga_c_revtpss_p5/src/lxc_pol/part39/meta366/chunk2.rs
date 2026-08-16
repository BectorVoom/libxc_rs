//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1281/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1281(t15339: f64, t954: f64, t4682: f64, t964: f64, t11404: f64, t11409: f64, t11507: f64, t11548: f64, t15263: f64, t15267: f64, t15274: f64, t15277: f64, t15280: f64, t15284: f64, t15287: f64, t15290: f64, t2943: f64, t2968: f64, t3007: f64, t3012: f64, t4652: f64, t4674: f64, t4685: f64, t946: f64, t974: f64) -> f64 {
    let t15340 = t15339 * t954;
    let t15343 = t4682 * t964;
    let t15348 = 0.17315859105681463759e2_f64 * t3012 * t15263 + 0.10254018858216406658e4_f64 * t11507 * t15267 - 4.0_f64 * t11548 * t4652 + 0.64327917994770140268e2_f64 * t11404 * t4674 - 4.0_f64 * t2943 * t15274 - 2.0_f64 * t2943 * t15277 - 0.19298375398431042081e3_f64 * t11409 * t15280 + 0.64327917994770140268e2_f64 * t2968 * t15284 + 6.0_f64 * t2968 * t15287 + 0.35089341735807877242e1_f64 * t3012 * t15290 + 1.0_f64 * t946 * t15340 + 0.11696447245269292414e1_f64 * t15343 * t974 + 0.5848223622634646207e0_f64 * t4685 * t3007;
    t15348
}
