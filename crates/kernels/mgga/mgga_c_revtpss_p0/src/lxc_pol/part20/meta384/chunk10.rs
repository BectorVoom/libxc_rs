//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1412/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1412<F: Float>(t10627: F, t198: F, t40067: F, t40072: F, t40155: F, t40157: F, t40160: F, t40163: F, t40167: F, t40171: F, t40173: F, t40175: F, t40179: F, t40184: F, t40187: F, t890: F, t892: F) -> F {
    let t41191 = F::cast_from(24.0_f64) * t10627 * t198 * t890 * t892 + t40067 - t40072 + t40155 - t40157 + t40160 + t40163 + t40167 - t40171 - t40173 + t40175 + t40179 - t40184 + t40187;
    t41191
}
