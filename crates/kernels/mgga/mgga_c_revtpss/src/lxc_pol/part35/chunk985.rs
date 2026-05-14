//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 985/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk985<F: Float>(t33: F, t265: F, t502: F, t30462: F, t1469: F, t2085: F, t30502: F, t57: F, t5825: F, t8059: F, t30470: F, t26405: F, t30122: F, t2047: F, t29532: F, t1923: F, t2048: F, t26175: F, t26207: F, t28154: F, t28598: F, t28600: F, t28602: F, t28628: F, t28638: F, t28641: F, t29513: F, t29538: F, t29544: F, t29548: F, t29551: F, t29554: F, t29562: F, t7343: F, t7702: F, t7706: F, t7709: F, t7964: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t30503 = piecewise3(t503, 0.0, t30462);
    let t30510 = piecewise3(t400, t30502, t30503 * t57 / 2.0 - t8059 * t1469 - t2085 * t5825 / 2.0);
    let t30511 = t30470 + t30510;
    let t30513 = t26405 * t30122;
    let t30543 = t2047 * t29532;
    let t30551 = -10.0 / 3.0 * t28602 * t7706 - 4.0 / 3.0 * t29538 * t2048 - 10.0 / 3.0 * t7343 * t29544 - 5.0 / 3.0 * t7343 * t29548 - 2.0 / 3.0 * t29551 * t2048 - 2.0 / 3.0 * t29554 * t2048 - 4.0 / 3.0 * t7709 * t7964 + 80.0 / 9.0 * t28598 + 32.0 / 9.0 * t28600 - 16.0 / 9.0 * t28638 + t29513 * t2048 / 3.0 + 2.0 / 3.0 * t7702 * t7964 + t1923 * t30543 / 3.0 - 16.0 / 9.0 * t28641 + 10.0 * t26175 * t29562 + 20.0 / 3.0 * t28154 * t28628 + t26207;
    (t30503, t30511, t30513, t30543, t30551)
}
