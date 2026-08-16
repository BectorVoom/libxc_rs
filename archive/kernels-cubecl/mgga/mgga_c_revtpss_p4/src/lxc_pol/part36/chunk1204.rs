//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1204/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1204<F: Float>(t33: F, t1469: F, t2159: F, t29977: F, t30936: F, t57: F, t5825: F, t8227: F, t30734: F, t118: F, t1502: F, t1843: F, t1911: F, t2127: F, t2163: F, t29497: F, t29501: F, t29504: F, t29507: F, t29510: F, t29512: F, t29578: F, t29580: F, t29582: F, t29585: F, t30716: F, t30724: F, t508: F, t5877: F, t5884: F, t6765: F, t8152: F, t8233: F, t8237: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t30943 = piecewise3::<F>(t400, t29977, t30936 * t57 / F::cast_from(2.0_f64) - t8227 * t1469 - t2159 * t5825 / F::cast_from(2.0_f64));
    let t30944 = t30734 + t30943;
    let t30950 = -t118 * t30944 - F::cast_from(2.0_f64) * t1502 * t8233 - F::cast_from(2.0_f64) * t1843 * t8152 + F::cast_from(2.0_f64) * t1911 * t8237 - t2127 * t6765 - t2163 * t5877 - F::cast_from(2.0_f64) * t2163 * t5884 - t30716 * t508 - F::cast_from(2.0_f64) * t30724 * t508 + t29497 + t29501 - t29504 + t29507 - t29510 - t29512 + t29578 + t29580 - t29582 + t29585;
    (t30944, t30950)
}
