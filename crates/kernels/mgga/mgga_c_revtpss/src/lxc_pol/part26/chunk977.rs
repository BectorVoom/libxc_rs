//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 977/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk977<F: Float>(t30: F, t265: F, t393: F, t26625: F, t2078: F, t2258: F, t26601: F, t45: F, t606: F, t7449: F, t1113: F, t1940: F, t2071: F, t2403: F, t25752: F, t25760: F, t25763: F, t25767: F, t25778: F, t25781: F, t25784: F, t26425: F, t26581: F, t26585: F, t26590: F, t33: F, t3351: F, t4541: F, t7200: F, t7207: F, t7428: F, t7432: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t26626 = piecewise3(t394, 0.0, t26625);
    let t26633 = piecewise3(t120, t26601, t26626 * t45 / 2.0 + t7449 * t606 + t2078 * t2258 / 2.0);
    let t26665 = 3.0 * t4541 * t2071 * t25752 + 3.0 * t2403 * t7428 * t7200 - 3.0 * t26425 * t25760 + 3.0 * t2403 * t2071 * t25763 + 3.0 / 2.0 * t2403 * t2071 * t25767 + t1940 * t26581 * t33 / 2.0 - t1940 * t26585 * t7207 + t1940 * t7428 * t1113 + t1940 * t26590 * t25778 - t1940 * t7432 * t25781 - t1940 * t7432 * t25784 / 2.0 + t1940 * t2071 * t3351 / 2.0;
    (t26626, t26633, t26665)
}
