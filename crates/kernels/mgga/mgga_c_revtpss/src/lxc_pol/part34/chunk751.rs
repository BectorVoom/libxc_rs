//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 751/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk751<F: Float>(t30: F, t1469: F, t1996: F, t45: F, t7794: F, t7856: F, t1544: F, t33: F, t1963: F, t1583: F, t1711: F, t1940: F, t2403: F, t7091: F, t7783: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t7861 = piecewise3::<f64>(t120, t7794, t1996 * t1469 / F::new(2.0) + t7856 * t45 / F::new(2.0));
    let t7862 = t33 * t1544;
    let t7863 = t1963 * t7862;
    let t7869 = t33 * t1583;
    let t7876 = F::new(3.0) / F::new(2.0) * t2403 * t7863 + t1940 * t7783 * t33 / F::new(2.0) - t1940 * t7091 * t7869 / F::new(2.0) + t1940 * t1963 * t1711 / F::new(2.0);
    (t7861, t7862, t7869, t7876)
}
