//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1827/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1827<F: Float>(t7036: F, t820: F, t844: F, t2482: F, t814: F, t10744: F, t2664: F, t7028: F, t25240: F, t2693: F, t2710: F, t228: F, t25273: F) -> (F, F, F, F, F) {
    let t92951 = t820 * t7036 * t844;
    let t92955 = t2482 * t7036 * t814;
    let t92963 = t10744 * t7028 * t2664;
    let t92966 = t2710 * t25240 * t2693;
    let t92968 = t25273 * t228;
    (t92951, t92955, t92963, t92966, t92968)
}
