//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 878/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk878<F: Float>(t1482: F, t16533: F, t542: F, t1477: F, t16194: F, t3255: F, t5432: F, t5436: F, t5442: F, t1419: F, t5808: F, t5498: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16534 = t1482 * t16533;
    let t16535 = t542 * t16534;
    let t16538 = t1477 * t16194;
    let t16539 = t542 * t16538;
    let t16543 = F::new(0.13140859333333333334e-2) * t3255 * t5432;
    let t16545 = F::new(0.8760572888888888889e-3) * t3255 * t5436;
    let t16547 = F::new(0.17521145777777777778e-2) * t3255 * t5442;
    let t16548 = t5808 * t1419;
    let t16549 = t5498 * t16548;
    (t16534, t16535, t16538, t16539, t16543, t16545, t16547, t16548, t16549)
}
