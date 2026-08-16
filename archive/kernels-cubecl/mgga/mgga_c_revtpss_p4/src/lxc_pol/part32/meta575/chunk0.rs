//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1901/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1901<F: Float>(t102218: F, t25895: F, t102204: F, t94771: F, t122: F, t72: F, t8085: F, t25900: F, t25899: F, t28894: F, t94921: F, t94802: F) -> (F, F, F, F, F, F, F) {
    let t102219 = t25895 * t102218;
    let t102225 = t94771 * t102204;
    let t102234 = t8085 * t72 * t122;
    let t102235 = t102234 * t25900;
    let t102237 = F::cast_from(0.25702851531048074406e-1_f64) * t25899 * t102235;
    let t102239 = F::cast_from(0.14456046980341999104e-1_f64) * t94921 * t28894;
    let t102241 = F::cast_from(0.25702851531048074406e-1_f64) * t94802 * t28894;
    (t102219, t102225, t102234, t102235, t102237, t102239, t102241)
}
