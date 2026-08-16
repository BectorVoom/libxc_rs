//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2020/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2020<F: Float>(t2466: F, t93329: F, t25375: F, t7015: F, t9292: F, t25411: F, t93183: F, t25431: F, t93123: F, t25387: F, t93285: F, t7063: F, t860: F) -> (F, F, F, F, F, F, F) {
    let t93330 = t93329 * t2466;
    let t93331 = t25375 * t93330;
    let t93334 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t7015;
    let t93335 = t25411 * t93183;
    let t93337 = t25431 * t93123;
    let t93339 = t25387 * t93285;
    let t93341 = t7063 * t860;
    (t93330, t93331, t93334, t93335, t93337, t93339, t93341)
}
