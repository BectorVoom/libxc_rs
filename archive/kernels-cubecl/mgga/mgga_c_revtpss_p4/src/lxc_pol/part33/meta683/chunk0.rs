//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2240/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2240<F: Float>(t104624: F, t104626: F, t104640: F, t104651: F, t104653: F, t20806: F, t20811: F, t20876: F, t21153: F, t21166: F, t21259: F, t26870: F, t26880: F, t29100: F, t6690: F, t7624: F, t97182: F) -> F {
    let t112175 = F::cast_from(0.28582678745379824648e-3_f64) * t26880 * t20811 - F::cast_from(0.85748036236139473944e-3_f64) * t97182 * t6690 - F::cast_from(0.85748036236139473944e-3_f64) * t26870 * t21166 - t104624 + t104626 + F::cast_from(0.57165357490759649296e-3_f64) * t26880 * t20876 - F::cast_from(0.28582678745379824648e-3_f64) * t7624 * t21153 - t104640 + t104651 - t104653 - F::cast_from(0.85748036236139473944e-3_f64) * t26870 * t21259 - F::cast_from(0.42874018118069736972e-3_f64) * t29100 * t20806;
    t112175
}
