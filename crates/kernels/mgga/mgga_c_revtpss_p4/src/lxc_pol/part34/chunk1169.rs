//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1169/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1169<F: Float>(t33: F, t265: F, t502: F, t29930: F, t1469: F, t2003: F, t29977: F, t57: F, t5825: F, t7877: F, t29938: F, t118: F, t1502: F, t1843: F, t1932: F, t2007: F, t29497: F, t29501: F, t29504: F, t29507: F, t29510: F, t29512: F, t29569: F, t29573: F, t29578: F, t29580: F, t29582: F, t29585: F, t29590: F, t508: F, t5877: F, t5884: F, t6765: F, t7725: F, t7883: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t29978 = piecewise3::<F>(t503, F::cast_from(0.0_f64), t29930);
    let t29985 = piecewise3::<F>(t400, t29977, t29978 * t57 / F::cast_from(2.0_f64) - t7877 * t1469 - t2003 * t5825 / F::cast_from(2.0_f64));
    let t29986 = t29938 + t29985;
    let t29991 = -t118 * t29986 - F::cast_from(2.0_f64) * t1502 * t7883 - F::cast_from(2.0_f64) * t1843 * t7725 - t1932 * t6765 - t2007 * t5877 - F::cast_from(2.0_f64) * t2007 * t5884 - t29569 * t508 - F::cast_from(2.0_f64) * t29573 * t508 + t29497 + t29501 - t29504 + t29507 - t29510 - t29512 + t29578 + t29580 - t29582 + t29585 - t29590;
    (t29978, t29986, t29991)
}
