//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1286/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1286<F: Float>(t100179: F, t101035: F, t101043: F, t101047: F, t101053: F, t101057: F, t26685: F, t26748: F, t27775: F, t27780: F, t27832: F, t28984: F, t28988: F, t93366: F, t95686: F) -> F {
    let t101060 = F::cast_from(0.46336805555555555556e-3_f64) * t26748 * t28984 - F::cast_from(0.7722800925925925926e-4_f64) * t101035 - F::cast_from(0.22109259259259259259e-2_f64) * t100179 - F::cast_from(0.27802083333333333334e-2_f64) * t27832 * t27775 - F::cast_from(0.13901041666666666667e-2_f64) * t27832 * t27780 + F::cast_from(0.2782641015625e-3_f64) * t26685 * t101043 + F::cast_from(0.10203017057291666667e-2_f64) * t26685 * t101047 - F::cast_from(0.18550940104166666667e-3_f64) * t93366 * t28988 - F::cast_from(0.556528203125e-3_f64) * t26685 * t101053 - F::cast_from(0.18550940104166666667e-3_f64) * t26685 * t101057 - t95686;
    t101060
}
