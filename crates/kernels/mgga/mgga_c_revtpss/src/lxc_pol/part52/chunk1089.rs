//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1089/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1089<F: Float>(t33: F, t265: F, t502: F, t25759: F, t34090: F, t27799: F, t34097: F, t1711: F, t1962: F, t34126: F, t1469: F, t1940: F, t2403: F, t26425: F, t28460: F, t28472: F, t32491: F, t33888: F, t34080: F, t57: F, t7432: F, t7862: F, t7869: F, t8657: F, t8677: F, t8682: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t34145 = t25759 * t34090;
    let t34151 = t27799 * t34097;
    let t34153 = t1711 * t1962;
    let t34161 = piecewise3::<F>(t503, F::new(0.0), t34126);
    let t34166 = piecewise3::<F>(t400, F::new(3.0) / F::new(2.0) * t2403 * t8657 * t7862 + t1940 * t34080 * t33 / F::new(2.0) - t1940 * t32491 * t7869 / F::new(2.0) + t1940 * t8657 * t1711 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t26425 * t34145 - t1940 * t28460 * t8677 / F::new(2.0) + t28472 * t34151 - t1940 * t7432 * t34153 / F::new(2.0) - t1940 * t7432 * t33888 / F::new(2.0), -t8682 * t1469 / F::new(2.0) + t34161 * t57 / F::new(2.0));
    (t34145, t34151, t34153, t34161, t34166)
}
