//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1986/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1986<F: Float>(t5876: F, t670: F, t1448: F, t6836: F, t6816: F, t1868: F, t5778: F, t10309: F, t607: F, t843: F, t1962: F, t41154: F) -> (F, F, F, F, F, F, F) {
    let t85360 = t5876 * t670;
    let t86753 = t6836 * t1448;
    let t86771 = t6816 * t1448;
    let t86815 = t1868 * t5778;
    let t92568 = t10309 * t607;
    let t92612 = F::new(1232.0) / F::new(27.0) * t843;
    let t92742 = t1962 * t41154;
    (t85360, t86753, t86771, t86815, t92568, t92612, t92742)
}
