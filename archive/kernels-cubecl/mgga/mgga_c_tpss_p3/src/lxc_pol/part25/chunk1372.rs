//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1372/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1372<F: Float>(t13603: F, t1692: F, t1812: F, t18728: F, t18807: F, t20018: F, t20048: F, t20510: F, t21485: F, t21510: F, t21513: F, t21710: F, t2439: F, t3552: F, t36547: F, t5059: F, t5678: F, t5849: F, t5853: F, t6207: F, t62829: F, t66317: F, t70813: F, t70861: F, t70872: F, t70893: F, t72188: F, t72265: F) -> F {
    let t72531 = t1692 * t1812 * t13603 / F::cast_from(2.0_f64) - t1692 * t5853 * t70861 / F::cast_from(2.0_f64) + t1692 * t62829 * t21510 + F::cast_from(2.0_f64) * t72188 * t20048 + F::cast_from(3.0_f64) * t3552 * t5849 * t21485 - t1692 * t72265 * t5678 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2439 * t20510 * t6207 + F::cast_from(3.0_f64) * t18728 * t70893 - F::cast_from(3.0_f64) * t66317 * t20018 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t18728 * t70813 - F::cast_from(3.0_f64) * t18728 * t70872 + F::cast_from(3.0_f64) * t36547 * t21710 - t1692 * t18807 * t21513 + t1692 * t5849 * t5059 / F::cast_from(2.0_f64);
    t72531
}
