//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1769/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1769<F: Float>(t213: F, t47343: F, t47348: F, t47351: F, t47352: F, t47354: F, t47359: F, t47364: F, t47369: F, t47375: F, t47379: F, t47381: F, t546: F) -> F {
    let t47383 = F::cast_from(0.65854491829355115987e0_f64) * t213 * t546 * t47343 + F::cast_from(0.78548797528808629095e-3_f64) * t47348 - t47351 + F::cast_from(0.1040793657534163522e-1_f64) * t47352 - F::cast_from(0.11708928647259339623e0_f64) * t47354 - F::cast_from(0.39029762157531132076e-1_f64) * t47359 - F::cast_from(0.69394917116090352835e-2_f64) * t47364 - F::cast_from(0.39029762157531132076e-1_f64) * t47369 - F::cast_from(0.23417857294518679245e0_f64) * t47375 + F::cast_from(0.23417857294518679245e0_f64) * t47379 - F::cast_from(0.44178176337912614788e-3_f64) * t47381;
    t47383
}
