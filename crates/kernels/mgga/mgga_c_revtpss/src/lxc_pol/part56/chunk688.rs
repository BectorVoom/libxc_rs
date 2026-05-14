//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 688/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk688<F: Float>(t30: F, t265: F, t393: F, t651: F, t8749: F, t8542: F, t45: F, t8498: F, t1936: F, t7586: F, t196: F, t2165: F, t197: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t8750 = t651 * t8749;
    let t8752 = piecewise3(t394, 0.0, t8542);
    let t8755 = piecewise3(t120, t8498, t8752 * t45 / 2.0);
    let t8758 = t7586 * t1936;
    let t8763 = t2165 * t196;
    let t8764 = t8763 * t197;
    (t8750, t8752, t8755, t8758, t8763, t8764)
}
