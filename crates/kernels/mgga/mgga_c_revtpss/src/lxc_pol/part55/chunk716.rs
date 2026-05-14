//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 716/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk716<F: Float>(t257: F, t827: F, t828: F, t1955: F, t239: F, t8464: F, t1954: F, t209: F, t2452: F) -> (F, F, F, F) {
    let t8468 = t827 * t828 * t257;
    let t8469 = t1955 * t8464 * t239 * t8468;
    let t8476 = t1954 * t209;
    let t8477 = t8476 * t2452;
    (t8468, t8469, t8476, t8477)
}
