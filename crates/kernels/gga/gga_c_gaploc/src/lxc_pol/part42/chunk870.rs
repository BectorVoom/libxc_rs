//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 870/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk870<F: Float>(t45516: F, t2976: F, t44787: F, t900: F, t13625: F, t22665: F, t7427: F, t2536: F, t3601: F, t2009: F, t2021: F, t13592: F, t2033: F, t549: F) -> (F, F, F, F, F) {
    let t45517 = F::new(0.14896037479937677779e-1) * t45516;
    let t45519 = t2976 * t900 * t44787;
    let t45520 = F::new(0.29792074959875355558e-1) * t45519;
    let t45522 = t7427 * t22665 * t13625;
    let t45524 = t2536 * t3601;
    let t45527 = F::new(0.35750489951850426669e0) * t2021 * t45524 * t2009;
    let t45529 = t2033 * t549 * t13592;
    (t45517, t45520, t45522, t45527, t45529)
}
