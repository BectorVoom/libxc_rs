//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 359/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk359<F: Float>(t30: F, t265: F, t393: F, t1102: F, t1587: F, t1598: F, t1612: F, t1638: F, t1640: F, t1644: F, t1699: F, t198: F, t336: F, t1468: F, t1469: F, t395: F, t45: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t1704 = piecewise3::<F>(t394, t1102 * t1699 * t198 * t336 - t1598 + t1612 + t1638 + t1640 - t1644, t1587);
    let t1709 = piecewise3::<F>(t120, t265 * t1468 / F::new(2.0) + t1587 * t30 / F::new(2.0), t395 * t1469 / F::new(2.0) + t1704 * t45 / F::new(2.0));
    (t1704, t1709)
}
