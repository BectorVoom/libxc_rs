//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3156/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3156(t1042: f64, t1261: f64, t17550: f64, t17569: f64, t20876: f64, t20880: f64, t24546: f64, t24668: f64, t44202: f64, t44526: f64, t5299: f64, t56246: f64, t57053: f64, t6619: f64, t69899: f64, t69910: f64, t69916: f64, t69968: f64, t78785: f64, t78790: f64) -> f64 {
    let t82929 = 0.57165357490759649295e-3_f64 * t69899 - 0.28582678745379824648e-3_f64 * t69910 + 0.60976381323476959248e-2_f64 * t69916 + 0.42874018118069736972e-2_f64 * t1261 * t1042 * t17550 * t78790 + 0.85748036236139473944e-2_f64 * t1261 * t1042 * t56246 * t78785 - 0.45732285992607719436e-2_f64 * t69968 * t5299 - 0.12862205435420921092e-2_f64 * t44526 * t24668 + 0.21437009059034868486e-3_f64 * t44202 * t24546 + 0.85748036236139473944e-3_f64 * t57053 * t6619 + 0.85748036236139473944e-3_f64 * t17569 * t20880 + 0.85748036236139473944e-3_f64 * t17569 * t20876;
    t82929
}
