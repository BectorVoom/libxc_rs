//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 359/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk359<F: Float>(t1188: F, t1756: F, t1161: F, t1180: F, t1721: F, t1735: F, t1737: F, t1745: F, t1750: F, t300: F, t435: F, t1179: F) -> (F, F, F, F) {
    let t1757 = t1756 * t1188;
    let t1761 = t300 * (-F::new(0.310907e-1) * t1737 * t435 + F::new(1.0) * t1161 * t1745 + t1721 - t1735 - F::new(0.19751673498613801407e-1) * t1750 + F::new(0.5848223622634646207e0) * t1180 * t1757);
    let t1763 = F::new(0.19751673498613801407e-1) * t300 * t1750;
    let t1765 = t1179 * t1756 * t1188;
    (t1757, t1761, t1763, t1765)
}
