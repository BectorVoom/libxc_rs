//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1162/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1162<F: Float>(t30: F, t113492: F, t114090: F, t1469: F, t1996: F, t22671: F, t29931: F, t45: F, t5825: F, t7856: F, t113440: F, t27799: F, t100987: F, t29598: F, t113103: F, t25759: F, t113432: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t114100 = piecewise3(t120, t113492, t114090 * t45 / 2.0 + 3.0 / 2.0 * t29931 * t1469 + 3.0 / 2.0 * t7856 * t5825 + t1996 * t22671 / 2.0);
    let t114101 = t27799 * t113440;
    let t114104 = t100987 * t29598;
    let t114107 = t25759 * t113103;
    let t114110 = t25759 * t113432;
    (t114100, t114101, t114104, t114107, t114110)
}
