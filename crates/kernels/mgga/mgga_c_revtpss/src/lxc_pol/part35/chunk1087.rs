//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1087/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1087<F: Float>(t30: F, t265: F, t393: F, t115819: F, t115462: F, t115763: F, t1469: F, t2078: F, t22671: F, t30463: F, t45: F, t5825: F, t8040: F, t102888: F, t103586: F, t110177: F, t113123: F, t114101: F, t114104: F, t114121: F, t114165: F, t114171: F, t114184: F, t114192: F, t114196: F, t1711: F, t1940: F, t2071: F, t2082: F, t2403: F, t26425: F, t26590: F, t28291: F, t28472: F, t29946: F, t29953: F, t29964: F, t30420: F, t6416: F, t7432: F, t7869: F, t8020: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t115820 = piecewise3(t394, 0.0, t115819);
    let t115830 = piecewise3(t120, t115462 + t115763, t115820 * t45 / 2.0 + 3.0 / 2.0 * t30463 * t1469 + 3.0 / 2.0 * t8040 * t5825 + t2078 * t22671 / 2.0);
    let t115870 = 9.0 * t26425 * t114101 - 9.0 * t26425 * t114104 - 9.0 * t102888 * t29946 - 9.0 * t28291 * t114165 + 3.0 * t28472 * t114196 + 3.0 / 2.0 * t1940 * t8020 * t6416 - 3.0 / 2.0 * t1940 * t7432 * t114184 + 3.0 / 2.0 * t2403 * t2071 * t114171 + 3.0 * t113123 * t2082 + 3.0 * t1940 * t26590 * t114121 + 9.0 / 2.0 * t2403 * t2071 * t114192 + 9.0 / 2.0 * t2403 * t8020 * t29953 - 3.0 / 2.0 * t1940 * t110177 * t7869 + 3.0 / 2.0 * t1940 * t30420 * t1711 + 3.0 * t1940 * t103586 * t29964;
    (t115830, t115870)
}
