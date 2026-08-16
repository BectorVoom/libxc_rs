//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1953/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1953(t33: f64, t265: f64, t502: f64, t29930: f64, t1469: f64, t2003: f64, t29977: f64, t57: f64, t5825: f64, t7877: f64, t29938: f64, t118: f64, t1502: f64, t1843: f64, t1932: f64, t2007: f64, t29497: f64, t29501: f64, t29504: f64, t29507: f64, t29510: f64, t29512: f64, t29569: f64, t29573: f64, t29578: f64, t29580: f64, t29582: f64, t29585: f64, t29590: f64, t508: f64, t5877: f64, t5884: f64, t6765: f64, t7725: f64, t7883: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t29978 = piecewise3(t503, 0.0_f64, t29930);
    let t29985 = piecewise3(t400, t29977, t29978 * t57 / 2.0_f64 - t7877 * t1469 - t2003 * t5825 / 2.0_f64);
    let t29986 = t29938 + t29985;
    let t29991 = -t118 * t29986 - 2.0_f64 * t1502 * t7883 - 2.0_f64 * t1843 * t7725 - t1932 * t6765 - t2007 * t5877 - 2.0_f64 * t2007 * t5884 - t29569 * t508 - 2.0_f64 * t29573 * t508 + t29497 + t29501 - t29504 + t29507 - t29510 - t29512 + t29578 + t29580 - t29582 + t29585 - t29590;
    (t29978, t29986, t29991)
}
