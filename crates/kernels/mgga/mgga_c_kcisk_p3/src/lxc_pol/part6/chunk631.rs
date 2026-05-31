//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 631/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk631<F: Float>(t4936: F, t4943: F, t7076: F, t7122: F, t8684: F, t8687: F, t8690: F, t8702: F, t8709: F, t8715: F, t8717: F, t8721: F, t8724: F, t8727: F) -> F {
    let t8763 = -F::cast_from(0.1294625e1_f64) * t8702 + F::cast_from(0.258925e1_f64) * t8709 + t4936 + F::cast_from(0.20128333333333333334e0_f64) * t7076 - F::cast_from(0.20128333333333333333e0_f64) * t8684 + F::cast_from(0.60385e0_f64) * t8687 - F::cast_from(0.301925e0_f64) * t8690 + F::cast_from(0.82524375e-1_f64) * t8715 + F::cast_from(0.16504875e0_f64) * t8717 + t4943 + F::cast_from(0.22076e0_f64) * t7122 - F::cast_from(0.5519e-1_f64) * t8721 + F::cast_from(0.33114e0_f64) * t8724 - F::cast_from(0.16557e0_f64) * t8727;
    t8763
}
