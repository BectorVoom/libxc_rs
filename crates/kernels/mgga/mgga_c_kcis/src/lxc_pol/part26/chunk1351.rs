//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1351/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1351<F: Float>(t1385: F, t28351: F, t58540: F, t101868: F, t103063: F, t103066: F, t103069: F, t103073: F, t103078: F, t16884: F, t27369: F, t3984: F, t59380: F, t59578: F, t7908: F, t7909: F, t7911: F, t97997: F, t98016: F) -> (F, F) {
    let t103083 = t28351 * t58540 * t1385;
    let t103095 = -F::new(0.2782641015625e-3) * t27369 * t103063 + F::new(0.41188271604938271607e-3) * t103066 - F::new(0.556528203125e-3) * t27369 * t103069 - F::new(0.46336805555555555557e-3) * t7908 * t103073 - F::new(0.22109259259259259259e-2) * t97997 - F::new(0.23168402777777777778e-3) * t103078 * t7911 + F::new(0.41188271604938271607e-3) * t98016 + F::new(0.41703125000000000001e-2) * t7908 * t103083 - F::new(0.33163888888888888888e-2) * t101868 + F::new(0.46336805555555555556e-3) * t7908 * t3984 * t7909 * t59578 - F::new(0.92673611111111111112e-3) * t7908 * t16884 * t7909 * t59380;
    (t103083, t103095)
}
