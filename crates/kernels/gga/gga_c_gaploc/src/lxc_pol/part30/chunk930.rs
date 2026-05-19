//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 930/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk930<F: Float>(t10144: F, t2343: F, t2268: F, t10115: F, t10118: F, t10119: F, t10124: F, t10127: F, t10131: F, t10134: F, t10137: F, t10139: F, t10143: F, t1063: F, t9072: F, t9077: F, t9085: F) -> (F, F) {
    let t10145 = t2343 * t10144;
    let t10147 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t10145;
    let t10148 = -t9072 + t9077 + t9085 + t10115 + t10118 - F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t10119 + F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t10124 + F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t10127 - t10131 - t10134 - t10137 + t10139 + t10143 + t10147;
    (t10145, t10148)
}
