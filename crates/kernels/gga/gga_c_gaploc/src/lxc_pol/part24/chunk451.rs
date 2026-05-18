//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 451/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk451<F: Float>(t213: F, t218: F, t1654: F, t847: F, t211: F, t215: F, t408: F, t608: F, t1666: F, t851: F, t220: F, t612: F, t43: F, t448: F, t894: F, zeta_threshold: F) -> (F, F) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t2214 = t1654 * t847;
    let t2217 = t215 * t211;
    let t2221 = piecewise3::<f64>(t214, F::new(0.0), F::new(4.0) / F::new(9.0) * t2214 * t608 + F::new(8.0) / F::new(3.0) * t2217 * t408);
    let t2222 = t1666 * t851;
    let t2225 = t220 * t211;
    let t2229 = piecewise3::<f64>(t219, F::new(0.0), F::new(4.0) / F::new(9.0) * t2222 * t612 - F::new(8.0) / F::new(3.0) * t2225 * t408);
    let t2231 = (t2221 + t2229) * t43;
    let t2264 = t894 * t448;
    (t2231, t2264)
}
