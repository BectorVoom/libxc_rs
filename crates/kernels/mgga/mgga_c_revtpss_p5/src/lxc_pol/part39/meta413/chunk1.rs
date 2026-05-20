//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1491/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1491<F: Float>(t31027: F, t31044: F, t2184: F, t4168: F, t31127: F, t571: F, t2192: F, t4153: F, t1455: F, t8302: F, t116: F, t31066: F) -> (F, F, F, F, F, F) {
    let t116995 = t31027 * t31044;
    let t117090 = t2184 * t4168;
    let t117095 = t571 * t31127;
    let t117097 = t4153 * t2192;
    let t117099 = t1455 * t8302;
    let t117103 = t116 * t31066;
    (t116995, t117090, t117095, t117097, t117099, t117103)
}
