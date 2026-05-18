//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1003/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1003<F: Float>(t30: F, t265: F, t393: F, t4248: F, t8749: F, t7732: F, t1936: F, t8233: F, t651: F, t33866: F, t1469: F, t33748: F, t45: F, t8752: F, t196: F, t197: F, t8237: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t34377 = t4248 * t8749;
    let t34379 = t7732 * t8749;
    let t34382 = t8233 * t1936;
    let t34383 = t651 * t34382;
    let t34388 = piecewise3::<f64>(t394, F::new(0.0), t33866);
    let t34393 = piecewise3::<f64>(t120, t33748, t8752 * t1469 / F::new(2.0) + t34388 * t45 / F::new(2.0));
    let t34399 = t8237 * t196 * t197;
    (t34377, t34379, t34382, t34383, t34388, t34393, t34399)
}
