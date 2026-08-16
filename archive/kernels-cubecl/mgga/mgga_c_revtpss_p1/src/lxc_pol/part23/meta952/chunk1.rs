//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3156/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3156<F: Float>(t1042: F, t1261: F, t17550: F, t17569: F, t20876: F, t20880: F, t24546: F, t24668: F, t44202: F, t44526: F, t5299: F, t56246: F, t57053: F, t6619: F, t69899: F, t69910: F, t69916: F, t69968: F, t78785: F, t78790: F) -> F {
    let t82929 = F::cast_from(0.57165357490759649295e-3_f64) * t69899 - F::cast_from(0.28582678745379824648e-3_f64) * t69910 + F::cast_from(0.60976381323476959248e-2_f64) * t69916 + F::cast_from(0.42874018118069736972e-2_f64) * t1261 * t1042 * t17550 * t78790 + F::cast_from(0.85748036236139473944e-2_f64) * t1261 * t1042 * t56246 * t78785 - F::cast_from(0.45732285992607719436e-2_f64) * t69968 * t5299 - F::cast_from(0.12862205435420921092e-2_f64) * t44526 * t24668 + F::cast_from(0.21437009059034868486e-3_f64) * t44202 * t24546 + F::cast_from(0.85748036236139473944e-3_f64) * t57053 * t6619 + F::cast_from(0.85748036236139473944e-3_f64) * t17569 * t20880 + F::cast_from(0.85748036236139473944e-3_f64) * t17569 * t20876;
    t82929
}
