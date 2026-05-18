//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 888/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk888<F: Float>(t4445: F, t6281: F, t1600: F, t1601: F, t6284: F, t2104: F, t4456: F, t286: F, t4318: F, t5469: F, t6939: F, t6942: F, t6946: F) -> (F, F, F, F, F, F, F, F) {
    let t7421 = t4445 * t6281;
    let t7422 = t1600 * t7421;
    let t7425 = t1601 * t6284;
    let t7426 = t1600 * t7425;
    let t7429 = t2104 * t2104;
    let t7430 = t4456 * t7429;
    let t7431 = t286 * t7430;
    let t7438 = t4318 + F::new(0.11415555555555555555e-1) * t5469 - F::new(0.11415555555555555555e-1) * t6939 + F::new(0.34246666666666666666e-1) * t6942 - F::new(0.17123333333333333333e-1) * t6946;
    (t7421, t7422, t7425, t7426, t7429, t7430, t7431, t7438)
}
