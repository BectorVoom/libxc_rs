//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2214/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2214<F: Float>(t60221: F, t7565: F, t13272: F, t26754: F, t101139: F, t101323: F, t101357: F, t2123: F, t25110: F, t25114: F, t28141: F, t29375: F, t29388: F, t6960: F, t6963: F, t7576: F, t7579: F, t7706: F, t96773: F, t96776: F) -> F {
    let t104279 = t60221 * t7565;
    let t104282 = t13272 * t26754;
    let t104303 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6963 * t29375 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t104279 * t6960 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t104282 * t6960 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t29388 * t25110 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t29388 * t25114 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t101323 * t2123 + t101357 * t2123 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28141 * t7576 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28141 * t7579 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t96773 * t7706 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t96776 * t7706 + t101139 * t2123 / F::cast_from(3.0_f64);
    t104303
}
