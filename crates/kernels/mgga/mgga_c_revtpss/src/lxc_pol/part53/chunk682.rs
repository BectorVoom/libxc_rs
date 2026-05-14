//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 682/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk682<F: Float>(t30: F, t33: F, t265: F, t502: F, t1469: F, t1996: F, t45: F, t7794: F, t7856: F, t1544: F, t1963: F, t1583: F, t1711: F, t1940: F, t2403: F, t7091: F, t7783: F, t7855: F, t2003: F, t57: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t7861 = piecewise3(t120, t7794, t1996 * t1469 / 2.0 + t7856 * t45 / 2.0);
    let t7862 = t33 * t1544;
    let t7863 = t1963 * t7862;
    let t7869 = t33 * t1583;
    let t7876 = 3.0 / 2.0 * t2403 * t7863 + t1940 * t7783 * t33 / 2.0 - t1940 * t7091 * t7869 / 2.0 + t1940 * t1963 * t1711 / 2.0;
    let t7877 = piecewise3(t503, 0.0, t7855);
    let t7882 = piecewise3(t400, t7876, -t2003 * t1469 / 2.0 + t7877 * t57 / 2.0);
    (t7861, t7862, t7869, t7876, t7877, t7882)
}
