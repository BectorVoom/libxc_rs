//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1131/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1131<F: Float>(t1098: F, t1103: F, t12361: F, t12368: F, t12371: F, t12385: F, t15544: F, t15547: F, t15550: F, t15554: F, t15558: F, t15561: F, t15564: F) -> F {
    let t15566 = t1098 * t15544 / F::cast_from(108.0_f64) - t1098 * t15547 / F::cast_from(72.0_f64) - t1098 * t15550 / F::cast_from(48.0_f64) - t12361 + t12368 / F::cast_from(10368.0_f64) - t12371 - F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t15554 * t1103 - t15558 / F::cast_from(432.0_f64) + t15561 / F::cast_from(648.0_f64) + t12385 / F::cast_from(648.0_f64) + F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t15564;
    t15566
}
