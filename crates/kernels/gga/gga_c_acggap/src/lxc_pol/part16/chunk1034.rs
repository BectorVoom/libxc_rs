//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1034/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1034<F: Float>(t1569: F, t7605: F, t13364: F, t31115: F, t35633: F, t1526: F, t2020: F, t2016: F, t8747: F, t7637: F, t8571: F, t1998: F, t5251: F) -> (F, F, F, F, F, F) {
    let t36372 = t7605 * t1569;
    let t36373 = F::new(0.34299214494455789578e-2) * t36372;
    let t36377 = t31115 * t13364 * t35633;
    let t36378 = F::new(0.10718504529517434243e-2) * t36377;
    let t36380 = t2020 * t1526;
    let t36381 = F::new(7.0) / F::new(144.0) * t36380;
    let t36382 = t2016 * t8747;
    let t36383 = F::new(0.28015625e-1) * t36382;
    let t36386 = t7637 * t8571;
    let t36388 = t1998 * t5251;
    (t36373, t36378, t36381, t36383, t36386, t36388)
}
