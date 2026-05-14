//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1054/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1054<F: Float>(t30: F, t265: F, t393: F, t1102: F, t1699: F, t198: F, t25713: F, t27712: F, t29894: F, t29930: F, t336: F, t5023: F, t6396: F, t6400: F, t7181: F, t1469: F, t1996: F, t29726: F, t45: F, t5825: F, t7856: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t29931 = piecewise3(t394, t1102 * t198 * t29894 * t336 - 2.0 * t1699 * t27712 * t5023 + 2.0 * t25713 * t5023 * t6400 - t5023 * t6396 * t7181, t29930);
    let t29938 = piecewise3(t120, t29726, t29931 * t45 / 2.0 + t7856 * t1469 + t1996 * t5825 / 2.0);
    (t29931, t29938)
}
