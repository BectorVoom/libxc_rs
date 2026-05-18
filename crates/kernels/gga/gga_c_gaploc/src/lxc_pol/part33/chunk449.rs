//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 449/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk449<F: Float>(t213: F, t218: F, t847: F, t1658: F, t90: F, t1654: F, t215: F, t851: F, t1666: F, t220: F, t43: F, t448: F, t894: F, t1061: F, t119: F, t481: F, zeta_threshold: F) -> (F, F, F) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t2237 = t847 * t847;
    let t2241 = F::new(2.0) * t90 + F::new(2.0) * t1658;
    let t2245 = piecewise3::<f64>(t214, F::new(0.0), F::new(4.0) / F::new(9.0) * t1654 * t2237 + F::new(4.0) / F::new(3.0) * t215 * t2241);
    let t2246 = t851 * t851;
    let t2249 = -t2241;
    let t2253 = piecewise3::<f64>(t219, F::new(0.0), F::new(4.0) / F::new(9.0) * t1666 * t2246 + F::new(4.0) / F::new(3.0) * t220 * t2249);
    let t2255 = (t2245 + t2253) * t43;
    let t2264 = t894 * t448;
    let t2268 = t481 * t1061 * t119;
    (t2255, t2264, t2268)
}
