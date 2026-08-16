//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 347/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk347(t1188: f64, t1756: f64, t1161: f64, t1180: f64, t1721: f64, t1735: f64, t1737: f64, t1745: f64, t1750: f64, t300: f64, t435: f64, t1179: f64) -> (f64, f64, f64, f64) {
    let t1757 = t1756 * t1188;
    let t1761 = t300 * (-0.310907e-1_f64 * t1737 * t435 + 1.0_f64 * t1161 * t1745 + t1721 - t1735 - 0.19751673498613801407e-1_f64 * t1750 + 0.5848223622634646207e0_f64 * t1180 * t1757);
    let t1763 = 0.19751673498613801407e-1_f64 * t300 * t1750;
    let t1765 = t1179 * t1756 * t1188;
    (t1757, t1761, t1763, t1765)
}
