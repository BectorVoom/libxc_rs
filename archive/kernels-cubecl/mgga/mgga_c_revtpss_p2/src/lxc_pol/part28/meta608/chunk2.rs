//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2109/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2109<F: Float>(t13977: F, t26028: F, t27928: F, t9775: F, t13775: F, t25986: F, t2661: F, t25978: F, t5614: F, t5622: F, t94443: F, t13769: F, t240: F, t7269: F) -> (F, F, F, F, F, F) {
    let t98135 = t26028 * t13977;
    let t98141 = t9775 * t27928;
    let t98144 = t2661 * t25986 * t13775;
    let t98145 = F::cast_from(0.28582678745379824648e-4_f64) * t98144;
    let t98146 = t25978 * t5614;
    let t98147 = F::cast_from(0.16006300097412701803e-1_f64) * t98146;
    let t98148 = t94443 * t5622;
    let t98152 = t2661 * t7269 * t240 * t13769;
    (t98135, t98141, t98145, t98147, t98148, t98152)
}
