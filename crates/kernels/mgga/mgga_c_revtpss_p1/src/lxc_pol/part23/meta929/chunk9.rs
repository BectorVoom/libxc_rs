//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3042/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3042<F: Float>(t30: F, t265: F, t393: F, t77472: F, t78403: F, t78414: F, t78444: F, t78475: F, t81075: F, t81076: F, t81078: F, t81088: F, t1106: F, t1468: F, t1469: F, t1587: F, t1704: F, t18280: F, t18281: F, t18884: F, t20236: F, t22670: F, t22671: F, t23436: F, t24192: F, t395: F, t4186: F, t45: F, t4560: F, t5028: F, t5824: F, t5825: F, t605: F, t606: F, t6405: F, t76396: F, t76397: F, t77481: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t81092 = piecewise3::<F>(t394, t78403 + t78414 + t78444 + t78475 + t81075 + t81076 + t81078 + t81088, t77472);
    let t81110 = piecewise3::<F>(t120, t77472 * t30 / F::new(2.0) + t23436 * t605 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t18884 * t1468 + t77481 + F::new(3.0) / F::new(2.0) * t4560 * t5824 + F::new(3.0) / F::new(2.0) * t1587 * t18280 + t895 * t22670 / F::new(2.0) + t265 * t76396 / F::new(2.0), t81092 * t45 / F::new(2.0) + t24192 * t606 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t20236 * t1469 + F::new(3.0) / F::new(2.0) * t6405 * t4186 + F::new(3.0) / F::new(2.0) * t5028 * t5825 + F::new(3.0) / F::new(2.0) * t1704 * t18281 + t1106 * t22671 / F::new(2.0) + t395 * t76397 / F::new(2.0));
    t81110
}
