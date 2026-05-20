//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 799/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk799<F: Float>(t30: F, t265: F, t393: F, t6084: F, t6404: F, t1468: F, t1469: F, t1587: F, t1704: F, t395: F, t45: F, t5824: F, t5825: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t6405 = piecewise3::<F>(t394, t6404, t6084);
    let t6412 = piecewise3::<F>(t120, t6084 * t30 / F::new(2.0) + t1587 * t1468 + t265 * t5824 / F::new(2.0), t6405 * t45 / F::new(2.0) + t1704 * t1469 + t395 * t5825 / F::new(2.0));
    (t6405, t6412)
}
