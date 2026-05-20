//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1900/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1900<F: Float>(t136: F, t2457: F, t8094: F, t94589: F, t26072: F, t28845: F, t28840: F, t686: F, t72: F, t25895: F, t2470: F, t28779: F) -> (F, F, F, F, F, F) {
    let t102204 = t8094 * t136 * t2457;
    let t102205 = t94589 * t102204;
    let t102213 = F::cast_from(0.14456046980341999104e-1_f64) * t26072 * t28845;
    let t102215 = t28840 * t72 * t686;
    let t102217 = F::cast_from(0.28912093960683998208e-1_f64) * t25895 * t102215;
    let t102218 = t28779 * t2470;
    (t102204, t102205, t102213, t102215, t102217, t102218)
}
