//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 626/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk626<F: Float>(t33: F, t265: F, t502: F, t6084: F, t6756: F, t1469: F, t1587: F, t1711: F, t1837: F, t504: F, t57: F, t5825: F, t6416: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t6757 = piecewise3::<f64>(t503, t6756, t6084);
    let t6764 = piecewise3::<f64>(t400, t6084 * t33 / F::new(2.0) + t1587 * t1711 + t265 * t6416 / F::new(2.0), t6757 * t57 / F::new(2.0) - t1837 * t1469 - t504 * t5825 / F::new(2.0));
    (t6757, t6764)
}
