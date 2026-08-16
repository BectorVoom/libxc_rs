//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2001/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2001<F: Float>(t843: F, t2247: F, t25138: F, t38: F, t1925: F, t2251: F, t45963: F, t6957: F, t10309: F, t25105: F, t45972: F, t45958: F) -> (F, F, F, F, F, F, F) {
    let t92612 = F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t843;
    let t92644 = t2247 * t38 * t25138;
    let t92666 = t2247 * t2251 * t1925;
    let t92684 = t45963 * t6957;
    let t92687 = t10309 * t25105;
    let t92690 = t45972 * t6957;
    let t92699 = t45958 * t6957;
    (t92612, t92644, t92666, t92684, t92687, t92690, t92699)
}
