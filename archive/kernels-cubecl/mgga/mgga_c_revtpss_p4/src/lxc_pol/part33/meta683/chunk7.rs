//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2247/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2247<F: Float>(t29037: F, t5378: F, t20786: F, t26849: F, t29010: F, t5265: F, t20819: F, t7617: F, t104696: F, t104793: F, t104815: F, t104817: F, t104825: F, t104828: F, t104833: F, t1252: F, t20797: F, t21046: F, t97261: F) -> F {
    let t112328 = t29037 * t5378;
    let t112334 = t26849 * t20786;
    let t112336 = t29010 * t5265;
    let t112339 = t20819 * t7617;
    let t112342 = -t104793 - F::cast_from(0.38110238327173099531e-3_f64) * t112328 + F::cast_from(0.42874018118069736972e-3_f64) * t104696 * t21046 + F::cast_from(0.42874018118069736972e-3_f64) * t97261 * t20797 - F::cast_from(0.28582678745379824648e-3_f64) * t112334 + F::cast_from(0.57165357490759649296e-3_f64) * t112336 - t104815 - t104817 + F::cast_from(0.19055119163586549765e-3_f64) * t104825 + t104828 + F::cast_from(0.42874018118069736972e-3_f64) * t112339 * t1252 + t104833;
    t112342
}
