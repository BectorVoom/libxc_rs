//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 291/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk291<F: Float>(t552: F, t938: F, t551: F, t910: F, t921: F, t595: F, t897: F, t602: F, t605: F) -> (F, F, F, F, F, F, F) {
    let t939 = t552 * t938;
    let t940 = t551 * t939;
    let t943 = t552 * t910;
    let t944 = t551 * t943;
    let t948 = t551 * t552 * t921;
    let t951 = t595 * t897;
    let t955 = F::new(12.0) * t602 + F::new(12.0) * t605;
    (t939, t940, t943, t944, t948, t951, t955)
}
