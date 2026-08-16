//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3232/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3232<F: Float>(t33: F, t265: F, t502: F, t77472: F, t81153: F, t81318: F, t81350: F, t81583: F, t81615: F, t81642: F, t84999: F, t85010: F, t1113: F, t1304: F, t1469: F, t1587: F, t1711: F, t18281: F, t1837: F, t18884: F, t20256: F, t21645: F, t22671: F, t22783: F, t23436: F, t25032: F, t4186: F, t4560: F, t504: F, t5509: F, t57: F, t5825: F, t606: F, t6416: F, t6757: F, t76397: F, t77481: F, t81123: F, t895: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t85014 = piecewise3::<F>(t503, t81153 + t81318 + t81350 + t81583 + t81615 + t81642 + t84999 + t85010, t77472);
    let t85032 = piecewise3::<F>(t400, t77472 * t33 / F::cast_from(2.0_f64) + t23436 * t1113 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t18884 * t1711 - t77481 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t4560 * t6416 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1587 * t20256 + t895 * t22783 / F::cast_from(2.0_f64) + t265 * t81123 / F::cast_from(2.0_f64), t85014 * t57 / F::cast_from(2.0_f64) - t25032 * t606 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t21645 * t1469 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t6757 * t4186 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t5509 * t5825 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1837 * t18281 - t1304 * t22671 / F::cast_from(2.0_f64) - t504 * t76397 / F::cast_from(2.0_f64));
    t85032
}
