//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1233/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1233<F: Float>(t30: F, t265: F, t393: F, t23436: F, t23560: F, t24190: F, t1468: F, t1469: F, t1587: F, t1704: F, t22670: F, t22671: F, t395: F, t45: F, t5824: F, t5825: F, t6084: F, t6405: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t24192 = piecewise3::<F>(t394, t23560 + t24190, t23436);
    let t24202 = piecewise3::<F>(t120, t23436 * t30 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t6084 * t1468 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1587 * t5824 + t265 * t22670 / F::cast_from(2.0_f64), t24192 * t45 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t6405 * t1469 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1704 * t5825 + t395 * t22671 / F::cast_from(2.0_f64));
    (t24192, t24202)
}
