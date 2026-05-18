//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1076/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1076<F: Float>(t529: F, t2153: F, t2308: F, t30490: F, t30498: F, t31679: F, t31695: F, t382: F, t525: F, t526: F, t6442: F, t8011: F, t8015: F, t8292: F) -> F {
    let t530 = t529 < -F::new(0.66725e-1);
    let t31702 = piecewise3::<f64>(t530, F::new(0.0), F::new(10.0) / F::new(9.0) * t525 * t31679 * t382 - F::new(10.0) / F::new(9.0) * t525 * t8292 * t2153 + F::new(40.0) / F::new(27.0) * t525 * t2308 * t8011 - F::new(10.0) / F::new(9.0) * t525 * t2308 * t8015 - F::new(280.0) / F::new(243.0) * t525 * t526 * t30490 + F::new(40.0) / F::new(27.0) * t6442 * t31695 - F::new(10.0) / F::new(27.0) * t525 * t526 * t30498);
    t31702
}
