//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 943/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk943<F: Float>(t33: F, t265: F, t502: F, t25759: F, t32498: F, t27799: F, t32505: F, t1113: F, t1962: F, t32534: F, t1940: F, t2403: F, t26425: F, t26585: F, t28472: F, t32080: F, t32487: F, t32491: F, t57: F, t606: F, t7200: F, t7207: F, t7432: F, t8657: F, t8677: F, t8682: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t32553 = t25759 * t32498;
    let t32559 = t27799 * t32505;
    let t32561 = t1113 * t1962;
    let t32569 = piecewise3(t503, 0.0, t32534);
    let t32574 = piecewise3(t400, 3.0 / 2.0 * t2403 * t8657 * t7200 + t1940 * t32487 * t33 / 2.0 - t1940 * t32491 * t7207 / 2.0 + t1940 * t8657 * t1113 / 2.0 - 3.0 / 2.0 * t26425 * t32553 - t1940 * t26585 * t8677 / 2.0 + t28472 * t32559 - t1940 * t7432 * t32561 / 2.0 - t1940 * t7432 * t32080 / 2.0, t32569 * t57 / 2.0 - t8682 * t606 / 2.0);
    (t32553, t32559, t32561, t32569, t32574)
}
