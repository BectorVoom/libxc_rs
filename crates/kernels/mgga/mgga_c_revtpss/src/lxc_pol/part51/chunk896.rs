//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 896/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk896<F: Float>(t33: F, t265: F, t502: F, t1711: F, t1940: F, t2403: F, t31863: F, t31876: F, t33727: F, t33888: F, t7091: F, t7862: F, t7869: F, t8490: F, t8494: F, t33866: F, t1469: F, t57: F, t8553: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t33896 = 3.0 / 2.0 * t2403 * t8490 * t7862 + t1940 * t33727 * t33 / 2.0 - t1940 * t31863 * t7869 / 2.0 + t1940 * t8490 * t1711 / 2.0 - 3.0 / 2.0 * t2403 * t8494 * t7862 - t1940 * t7091 * t33888 + t1940 * t31876 * t7869 - t1940 * t8494 * t1711 / 2.0;
    let t33897 = piecewise3(t503, 0.0, t33866);
    let t33902 = piecewise3(t400, t33896, -t8553 * t1469 / 2.0 + t33897 * t57 / 2.0);
    (t33897, t33902)
}
