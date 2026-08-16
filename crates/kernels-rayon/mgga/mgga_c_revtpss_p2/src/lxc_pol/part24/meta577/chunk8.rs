//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1777/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1777(t12429: f64, t12486: f64, t12553: f64, t17023: f64, t17032: f64, t1744: f64, t1756: f64, t20678: f64, t24363: f64, t24408: f64, t24417: f64, t24420: f64, t3452: f64, t3477: f64, t3496: f64, t3521: f64, t6487: f64, t6502: f64, t6506: f64, t6519: f64, t6534: f64, t6538: f64, t81873: f64, t90346: f64, t90349: f64, t90351: f64, t90364: f64, t90367: f64, t90370: f64, t90373: f64) -> f64 {
    let t90805 = -t90346 + t90349 - t90351 - 0.62337092780453269531e3_f64 * t12486 * t6538 * t6534 - 0.46785788981077169656e1_f64 * t3496 * t24408 * t1756 + 0.69263436422725855036e2_f64 * t3521 * t81873 * t1756 + 0.61524113149298439947e4_f64 * t12553 * t20678 * t6534 + 36.0_f64 * t3477 * t6487 * t6502 + 0.21053605041484726346e2_f64 * t3521 * t6519 * t6534 + t90364 + t90367 - t90370 - t90373 - 24.0_f64 * t17023 * t24417 + 0.3859675079686208416e3_f64 * t17032 * t24420 - 0.11579025239058625248e4_f64 * t12429 * t6506 * t6502 - 8.0_f64 * t3452 * t24363 * t1744;
    t90805
}
