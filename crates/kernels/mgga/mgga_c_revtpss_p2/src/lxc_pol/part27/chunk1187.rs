//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1187/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1187<F: Float>(t644: F, t6977: F, t25113: F, t77: F, t1927: F, t2315: F, t2247: F, t2259: F, t843: F, t10406: F, t76: F, t38: F, t45955: F) -> (F, F, F, F, F, F, F) {
    let t92576 = t6977 * t644;
    let t92581 = t77 * t25113 * t644;
    let t92584 = t1927 * t2315;
    let t92588 = t2247 * t2259;
    let t92612 = F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t843;
    let t92628 = t76 * t10406;
    let t92632 = t45955 * t38;
    (t92576, t92581, t92584, t92588, t92612, t92628, t92632)
}
