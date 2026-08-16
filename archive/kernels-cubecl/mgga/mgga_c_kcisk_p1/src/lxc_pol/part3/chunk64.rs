//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 64/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk64<F: Float>(t60: F, t180: F, t182: F, t183: F) -> (F, F) {
    let t187 = t60 * t60;
    let t189 = F::cast_from(0.19711288999999999999e-2_f64) * t180 * t182 * t183 - F::cast_from(2.0_f64) * t187;
    let t190 = F::cast_from(1.0_f64) / t189;
    (t189, t190)
}
