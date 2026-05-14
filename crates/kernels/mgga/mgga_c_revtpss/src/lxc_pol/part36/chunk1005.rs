//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1005/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1005<F: Float>(t33: F, t265: F, t502: F, t23436: F, t24476: F, t25030: F, t1469: F, t1587: F, t1711: F, t1837: F, t22671: F, t22783: F, t504: F, t57: F, t5825: F, t6084: F, t6416: F, t6757: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t25032 = piecewise3(t503, t24476 + t25030, t23436);
    let t25042 = piecewise3(t400, t23436 * t33 / 2.0 + 3.0 / 2.0 * t6084 * t1711 + 3.0 / 2.0 * t1587 * t6416 + t265 * t22783 / 2.0, t25032 * t57 / 2.0 - 3.0 / 2.0 * t6757 * t1469 - 3.0 / 2.0 * t1837 * t5825 - t504 * t22671 / 2.0);
    (t25042,)
}
