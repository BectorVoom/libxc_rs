//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1226/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1226<F: Float>(t239: F, t820: F, t94491: F, t2482: F, t596: F, t7262: F, t25981: F, t27: F, t550: F, t7021: F, t1412: F, t1941: F) -> (F, F, F, F, F) {
    let t94493 = t820 * t94491 * t239;
    let t94497 = t2482 * t7262 * t596;
    let t94508 = t2482 * t25981 * t27;
    let t94513 = t7021 * t550;
    let t94516 = t1941 * t1412;
    (t94493, t94497, t94508, t94513, t94516)
}
