//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 774/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk774<F: Float>(t30: F, t1469: F, t2129: F, t45: F, t7794: F, t8161: F, t1479: F, t343: F, t136: F, t1785: F, t2138: F, t1802: F, t2137: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t8166 = piecewise3::<f64>(t120, t7794, t2129 * t1469 / F::new(2.0) + t8161 * t45 / F::new(2.0));
    let t8171 = t1479 * t343;
    let t8172 = t8171 * t136;
    let t8177 = t1785 * t2138;
    let t8184 = t2137 * t1802;
    (t8166, t8171, t8172, t8177, t8184)
}
