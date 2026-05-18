//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 611/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk611<F: Float>(t33: F, t265: F, t502: F, t4560: F, t5508: F, t1113: F, t1304: F, t1469: F, t1587: F, t1711: F, t1837: F, t4186: F, t4568: F, t504: F, t57: F, t606: F, t895: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t5509 = piecewise3::<f64>(t503, t5508, t4560);
    let t5516 = piecewise3::<f64>(t400, t4560 * t33 / F::new(2.0) + t1587 * t1113 / F::new(2.0) + t895 * t1711 / F::new(2.0) - t4568, -t1304 * t1469 / F::new(2.0) - t1837 * t606 / F::new(2.0) - t504 * t4186 / F::new(2.0) + t5509 * t57 / F::new(2.0));
    t5516
}
