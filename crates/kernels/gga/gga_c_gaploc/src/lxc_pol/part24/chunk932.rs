//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 932/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk932<F: Float>(t10140: F, t2343: F, t2268: F, t2293: F, t2787: F) -> (F, F, F) {
    let t10141 = t2343 * t10140;
    let t10143 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t10141;
    let t10144 = t2787 * t2293;
    (t10141, t10143, t10144)
}
