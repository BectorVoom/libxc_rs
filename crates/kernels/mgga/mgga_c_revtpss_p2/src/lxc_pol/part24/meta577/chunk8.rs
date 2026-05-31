//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1777/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1777<F: Float>(t12429: F, t12486: F, t12553: F, t17023: F, t17032: F, t1744: F, t1756: F, t20678: F, t24363: F, t24408: F, t24417: F, t24420: F, t3452: F, t3477: F, t3496: F, t3521: F, t6487: F, t6502: F, t6506: F, t6519: F, t6534: F, t6538: F, t81873: F, t90346: F, t90349: F, t90351: F, t90364: F, t90367: F, t90370: F, t90373: F) -> F {
    let t90805 = -t90346 + t90349 - t90351 - F::cast_from(0.62337092780453269531e3_f64) * t12486 * t6538 * t6534 - F::cast_from(0.46785788981077169656e1_f64) * t3496 * t24408 * t1756 + F::cast_from(0.69263436422725855036e2_f64) * t3521 * t81873 * t1756 + F::cast_from(0.61524113149298439947e4_f64) * t12553 * t20678 * t6534 + F::cast_from(36.0_f64) * t3477 * t6487 * t6502 + F::cast_from(0.21053605041484726346e2_f64) * t3521 * t6519 * t6534 + t90364 + t90367 - t90370 - t90373 - F::cast_from(24.0_f64) * t17023 * t24417 + F::cast_from(0.3859675079686208416e3_f64) * t17032 * t24420 - F::cast_from(0.11579025239058625248e4_f64) * t12429 * t6506 * t6502 - F::cast_from(8.0_f64) * t3452 * t24363 * t1744;
    t90805
}
