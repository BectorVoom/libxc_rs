//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1113/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1113<F: Float>(t81437: F, t39063: F, t7025: F, t23966: F, t9239: F, t2240: F, t240: F, t33: F, t1860: F, t1864: F, t67: F, t835: F) -> (F, F, F, F, F) {
    let t84036 = F::cast_from(308.0_f64) / F::cast_from(27.0_f64) * t81437;
    let t84216 = t39063 * t7025;
    let t84219 = t9239 * t23966;
    let t84241 = t2240 * t33 * t240;
    let t84280 = F::cast_from(1232.0_f64) / F::cast_from(81.0_f64) * t1860 * t835 * t67 * t1864;
    (t84036, t84216, t84219, t84241, t84280)
}
