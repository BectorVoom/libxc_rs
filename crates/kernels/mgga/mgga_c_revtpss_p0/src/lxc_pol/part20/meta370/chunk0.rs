//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1346/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1346<F: Float>(t40178: F, t40067: F, t40072: F, t40155: F, t40157: F, t40160: F, t40163: F, t40167: F, t40171: F, t40173: F, t40175: F, t39909: F, t738: F, t745: F) -> (F, F, F) {
    let t40179 = F::cast_from(144.0_f64) * t40178;
    let t40180 = t40155 - t40157 + t40067 - t40072 + t40160 + t40163 + t40167 - t40171 - t40173 + t40175 + t40179;
    let t40182 = t738 * t39909 * t745;
    (t40179, t40180, t40182)
}
