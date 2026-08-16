//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 818/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk818<F: Float>(t114: F, t1163: F, t1799: F, t5525: F, t5528: F) -> (F, F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t5809 = t1163 * t1799;
    let t5812 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5525;
    let t5815 = piecewise3::<F>(t115, F::cast_from(0.0_f64), -t5812 - t5528 / F::cast_from(4.0_f64));
    (t5809, t5812, t5815)
}
