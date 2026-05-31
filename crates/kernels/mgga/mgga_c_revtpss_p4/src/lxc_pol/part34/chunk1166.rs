//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1166/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1166<F: Float>(t265: F, t393: F, t1102: F, t1699: F, t198: F, t25713: F, t27712: F, t29894: F, t29930: F, t336: F, t5023: F, t6396: F, t6400: F, t7181: F) -> F {
    let t394 = t265 < t393;
    let t29931 = piecewise3::<F>(t394, t1102 * t198 * t29894 * t336 - F::cast_from(2.0_f64) * t1699 * t27712 * t5023 + F::cast_from(2.0_f64) * t25713 * t5023 * t6400 - t5023 * t6396 * t7181, t29930);
    t29931
}
