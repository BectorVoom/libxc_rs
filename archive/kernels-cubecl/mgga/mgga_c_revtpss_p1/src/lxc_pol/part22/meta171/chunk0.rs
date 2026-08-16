//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1129/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1129<F: Float>(t4029: F, t512: F, t1320: F, t1331: F, t1340: F, t2516: F) -> (F, F, F, F, F) {
    let t4030 = t512 * t4029;
    let t4031 = F::cast_from(2.0_f64) * t4030;
    let t4032 = t1320 * t1331;
    let t4033 = F::cast_from(8.0_f64) * t4032;
    let t4035 = F::cast_from(0.5848223622634646207e0_f64) * t1340 * t2516;
    (t4030, t4031, t4032, t4033, t4035)
}
