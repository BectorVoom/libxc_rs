//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1977/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1977<F: Float>(t1448: F, t6816: F, t1868: F, t5778: F, t10309: F, t607: F, t2275: F, t613: F, t10355: F, t43: F, t843: F, t45963: F, t6957: F) -> (F, F, F, F, F, F, F) {
    let t86771 = t6816 * t1448;
    let t86815 = t1868 * t5778;
    let t92568 = t10309 * t607;
    let t92600 = t613 * t2275;
    let t92605 = t43 * t10355;
    let t92612 = F::new(1232.0) / F::new(27.0) * t843;
    let t92684 = t45963 * t6957;
    (t86771, t86815, t92568, t92600, t92605, t92612, t92684)
}
