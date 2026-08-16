//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1070/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1070<F: Float>(t28029: F, t5177: F, t5078: F, t7754: F, t26930: F, t5099: F, t5062: F, t7748: F, t1200: F, t4999: F, t28012: F, t28014: F, t28016: F, t28018: F, t28020: F, t28022: F, t28025: F, t28027: F) -> (F, F, F, F, F, F) {
    let t28030 = t28029 * t5177;
    let t28032 = t7754 * t5078;
    let t28034 = t26930 * t5099;
    let t28036 = t7748 * t5062;
    let t28038 = t4999 * t1200;
    let t28040 = -t28012 / F::cast_from(6.0_f64) + t28014 / F::cast_from(16.0_f64) - t28016 / F::cast_from(128.0_f64) + t28018 / F::cast_from(24.0_f64) - t28020 / F::cast_from(24.0_f64) + t28022 / F::cast_from(18.0_f64) - t28025 / F::cast_from(288.0_f64) + t28027 / F::cast_from(128.0_f64) - t28030 / F::cast_from(64.0_f64) - t28032 / F::cast_from(72.0_f64) + t28034 / F::cast_from(96.0_f64) - t28036 / F::cast_from(24.0_f64) - t28038 / F::cast_from(96.0_f64);
    (t28030, t28032, t28034, t28036, t28038, t28040)
}
