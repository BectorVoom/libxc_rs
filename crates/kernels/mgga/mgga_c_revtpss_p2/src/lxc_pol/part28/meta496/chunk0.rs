//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1877/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1877<F: Float>(t25986: F, t3994: F, t2661: F, t3970: F, t7271: F, t4014: F, t4059: F, t7264: F, t2482: F, t27: F, t7262: F) -> (F, F, F, F, F, F, F) {
    let t25987 = t25986 * t3994;
    let t25988 = t2661 * t25987;
    let t25989 = F::cast_from(0.28582678745379824648e-4_f64) * t25988;
    let t25990 = t7271 * t3970;
    let t25992 = t7271 * t4014;
    let t25994 = t7264 * t4059;
    let t25997 = t2482 * t7262 * t27;
    (t25987, t25988, t25989, t25990, t25992, t25994, t25997)
}
