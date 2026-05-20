//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1531/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1531<F: Float>(t11651: F, t11659: F, t11774: F, t11776: F, t11866: F, t11871: F, t16025: F, t3096: F, t3117: F, t3120: F, t372: F, t42315: F, t43029: F, t43032: F, t43035: F, t43038: F, t43044: F, t43050: F, t43051: F, t43057: F, t43063: F, t43066: F, t43069: F) -> F {
    let t43074 = -t43029 / F::new(36.0) + t43032 / F::new(54.0) - F::cast_from(0.17149607247227894789e-2_f64) * t43035 - F::cast_from(0.25724410870841842184e-2_f64) * t43038 * t3120 - F::cast_from(0.25724410870841842184e-2_f64) * t11866 * t11871 - F::cast_from(0.25724410870841842184e-2_f64) * t43044 * t3117 * t11659 * t16025 + F::cast_from(0.51448821741683684368e-2_f64) * t43050 * t3117 * t11659 * t43051 - F::cast_from(0.28582678745379824648e-2_f64) * t11774 * t42315 * t43057 - F::cast_from(0.22866142996303859719e-2_f64) * t43063 + F::cast_from(0.18292914397043087775e-1_f64) * t43066 * t11776 + F::cast_from(0.34299214494455789578e-2_f64) * t43069 * t372 * t11651 * t3096;
    t43074
}
