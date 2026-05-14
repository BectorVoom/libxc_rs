//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 977/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk977<F: Float>(t30: F, t265: F, t393: F, t34126: F, t1468: F, t1469: F, t1940: F, t2403: F, t26425: F, t28460: F, t28472: F, t32491: F, t33740: F, t34080: F, t34091: F, t34098: F, t34100: F, t45: F, t7432: F, t7749: F, t7787: F, t8657: F, t8660: F, t8671: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t34127 = piecewise3(t394, 0.0, t34126);
    let t34132 = piecewise3(t120, 3.0 / 2.0 * t2403 * t8657 * t7749 + t1940 * t34080 * t30 / 2.0 - t1940 * t32491 * t7787 / 2.0 + t1940 * t8657 * t1468 / 2.0 - 3.0 / 2.0 * t26425 * t34091 - t1940 * t28460 * t8660 / 2.0 + t28472 * t34098 - t1940 * t7432 * t34100 / 2.0 - t1940 * t7432 * t33740 / 2.0, t8671 * t1469 / 2.0 + t34127 * t45 / 2.0);
    (t34127, t34132)
}
