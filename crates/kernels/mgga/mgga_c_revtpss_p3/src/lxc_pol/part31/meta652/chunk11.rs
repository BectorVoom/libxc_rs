//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2175/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2175<F: Float>(t30: F, t265: F, t393: F, t107820: F, t107867: F, t107772: F, t106638: F, t1469: F, t18281: F, t1996: F, t27755: F, t29931: F, t4186: F, t45: F, t5825: F, t606: F, t7194: F, t7856: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t107868 = t107820 + t107867;
    let t107869 = piecewise3::<F>(t394, t107772, t107868);
    let t107881 = piecewise3::<F>(t120, t106638, t107869 * t45 / F::cast_from(2.0_f64) + t29931 * t606 / F::cast_from(2.0_f64) + t27755 * t1469 + t7856 * t4186 + t7194 * t5825 / F::cast_from(2.0_f64) + t1996 * t18281 / F::cast_from(2.0_f64));
    (t107868, t107881)
}
