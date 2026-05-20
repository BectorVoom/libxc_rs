//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1826/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1826<F: Float>(t5876: F, t670: F, t1448: F, t6836: F, t6816: F, t1868: F, t5778: F, t10309: F, t607: F, t2411: F, t605: F, t1955: F, t25308: F, t2769: F) -> (F, F, F, F, F, F, F) {
    let t85360 = t5876 * t670;
    let t86753 = t6836 * t1448;
    let t86771 = t6816 * t1448;
    let t86815 = t1868 * t5778;
    let t92568 = t10309 * t607;
    let t92790 = t2411 * t605;
    let t92917 = t1955 * t25308 * t2769;
    (t85360, t86753, t86771, t86815, t92568, t92790, t92917)
}
