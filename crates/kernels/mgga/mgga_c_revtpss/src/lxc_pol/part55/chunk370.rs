//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 370/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk370<F: Float>(t33: F, t265: F, t502: F, t1300: F, t1587: F, t1721: F, t1735: F, t1761: F, t1763: F, t1767: F, t1832: F, t198: F, t336: F, t1469: F, t1711: F, t504: F, t57: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t1837 = piecewise3::<f64>(t503, t1300 * t1832 * t198 * t336 - t1721 + t1735 + t1761 + t1763 - t1767, t1587);
    let t1842 = piecewise3::<f64>(t400, t1587 * t33 / F::new(2.0) + t265 * t1711 / F::new(2.0), -t504 * t1469 / F::new(2.0) + t1837 * t57 / F::new(2.0));
    (t1837, t1842)
}
