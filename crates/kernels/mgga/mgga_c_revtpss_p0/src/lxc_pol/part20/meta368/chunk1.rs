//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1343/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1343<F: Float>(t40148: F, t10565: F, t717: F, t39989: F, t40126: F, t40128: F, t40131: F, t40133: F, t40137: F, t40140: F, t40142: F, t40144: F, t40146: F) -> (F, F, F) {
    let t40149 = F::cast_from(96.0_f64) * t40148;
    let t40150 = t717 * t10565;
    let t40151 = F::cast_from(4.0_f64) * t40150;
    let t40152 = -t40126 + t40128 - t40131 - t40133 - t40137 + t40140 + t40142 + t40144 + t40146 + t40149 - t39989 + t40151;
    (t40149, t40151, t40152)
}
