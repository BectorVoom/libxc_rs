//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 534/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk534<F: Float>(t1592: F, t4419: F, t535: F, t3571: F, t3573: F, t3577: F, t3581: F, t3585: F, t1524: F, t1528: F, t1527: F, t512: F) -> (F, F, F, F, F) {
    let t4420 = t4419 * t1592;
    let t4421 = t535 * t4420;
    let t4423 = F::new(0.22831111111111111111e-1) * t3571;
    let t4428 = t4423 + F::new(0.11415555555555555555e-1) * t3573 - F::new(0.11415555555555555555e-1) * t3577 + F::new(0.34246666666666666666e-1) * t3581 - F::new(0.17123333333333333333e-1) * t3585;
    let t4431 = t1524 * t1528;
    let t4434 = t1527 * t512;
    (t4420, t4421, t4428, t4431, t4434)
}
