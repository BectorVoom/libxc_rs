//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 122/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk122<F: Float>(t28: F, t14: F, t167: F, t2: F, t4: F, t7: F) -> (F, F, F, F, F) {
    let t382 = t28 * t28;
    let t383 = F::new(1.0) / t382;
    let t384 = t14 * t383;
    let t385 = t167 * t2;
    let t386 = t4 * t7;
    (t382, t383, t384, t385, t386)
}
