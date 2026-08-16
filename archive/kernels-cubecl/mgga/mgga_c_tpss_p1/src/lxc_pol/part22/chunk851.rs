//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 851/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk851<F: Float>(t5709: F, t5909: F, t5714: F, t5724: F, t5717: F, t5722: F, t5729: F) -> (F, F, F, F) {
    let t5910 = t5909 * t5709;
    let t5913 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t5714;
    let t5916 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t5724;
    let t5918 = -t5913 - t5717 / F::cast_from(24.0_f64) - t5722 / F::cast_from(768.0_f64) - t5916 - t5729 / F::cast_from(192.0_f64);
    (t5910, t5913, t5916, t5918)
}
