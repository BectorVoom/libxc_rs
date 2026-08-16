//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 750/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk750<F: Float>(t9062: F, t9066: F, t9150: F, t9152: F, t9155: F, t9158: F, t9163: F, t9166: F, t9168: F, t9170: F, t9173: F, t9176: F, t9179: F, t9182: F) -> F {
    let t9311 = -t9062 / F::cast_from(8.0_f64) - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t9066 + t9150 / F::cast_from(8.0_f64) - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t9152 + F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t9155 + t9158 / F::cast_from(64.0_f64) + F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t9163 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t9166 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t9168 + F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t9170 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t9173 - t9176 / F::cast_from(64.0_f64) + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t9179 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t9182;
    t9311
}
