//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2319/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2319<F: Float>(t18324: F, t7310: F, t18371: F, t24741: F, t29569: F, t29651: F, t4954: F, t7321: F, t86184: F, t95320: F, t95334: F, t95335: F, t95352: F, t95362: F, t95364: F, t95365: F, t95687: F) -> F {
    let t104088 = t7310 * t18324;
    let t104094 = t24741 * t18371;
    let t104101 = -t104088 / F::cast_from(432.0_f64) - F::cast_from(0.16149102437656156342e-2_f64) * t29569 * t7321 + t95320 - t95687 * t4954 / F::cast_from(1152.0_f64) - t104094 / F::cast_from(1728.0_f64) - t95334 - t95335 / F::cast_from(3456.0_f64) + F::cast_from(0.10093189023535097714e-3_f64) * t29651 * t7321 + t86184 / F::cast_from(1296.0_f64) - t95352 + t95362 - t95364 - t95365 / F::cast_from(3456.0_f64);
    t104101
}
