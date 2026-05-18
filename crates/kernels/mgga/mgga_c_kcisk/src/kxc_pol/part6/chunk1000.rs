//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1000/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1000<F: Float>(t385: F, t1284: F, t30205: F, t2147: F, t2153: F, t30476: F, t30490: F, t30494: F, t340: F, t379: F, t382: F, t6141: F, t6142: F, t8003: F, t8011: F, t8015: F) -> (F, F) {
    let t386 = t385 < -F::new(0.66725e-1);
    let t30498 = t1284 * t30205;
    let t30503 = piecewise3::<f64>(t386, F::new(0.0), F::new(10.0) / F::new(9.0) * t340 * t30476 * t382 - F::new(10.0) / F::new(9.0) * t340 * t8003 * t2153 + F::new(40.0) / F::new(27.0) * t340 * t2147 * t8011 - F::new(10.0) / F::new(9.0) * t340 * t2147 * t8015 - F::new(280.0) / F::new(243.0) * t340 * t379 * t30490 + F::new(40.0) / F::new(27.0) * t6141 * t6142 * t30494 - F::new(10.0) / F::new(27.0) * t340 * t379 * t30498);
    (t30498, t30503)
}
