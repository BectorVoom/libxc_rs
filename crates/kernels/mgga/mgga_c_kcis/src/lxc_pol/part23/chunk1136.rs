//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1136/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1136<F: Float>(t1380: F, t167: F, t1650: F, t4007: F, t3977: F, t498: F, t12133: F, t16848: F, t12159: F, t613: F, t1938: F, t12230: F, t1924: F) -> (F, F, F, F, F, F, F, F) {
    let t52371 = t167 * t1380;
    let t52402 = t1650 * t4007;
    let t52460 = t3977 * t498;
    let t52613 = t12133 * t498;
    let t52649 = t16848 * t498;
    let t52696 = t613 * t12159;
    let t52697 = t1938 * t1380;
    let t52852 = t1924 * t12230;
    (t52371, t52402, t52460, t52613, t52649, t52696, t52697, t52852)
}
