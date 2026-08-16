//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1833/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1833<F: Float>(t2482: F, t25260: F, t27: F, t10852: F, t25266: F, t2756: F, t10836: F, t25227: F, t2661: F, t596: F, t7036: F, t2487: F) -> (F, F, F, F, F) {
    let t93025 = t2482 * t25260 * t27;
    let t93026 = t93025 * t10852;
    let t93028 = t25266 * t2756;
    let t93031 = t2661 * t25227 * t10836;
    let t93034 = t2482 * t7036 * t596;
    let t93035 = t93034 * t2487;
    (t93026, t93028, t93031, t93034, t93035)
}
