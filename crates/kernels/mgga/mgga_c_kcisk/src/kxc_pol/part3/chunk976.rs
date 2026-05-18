//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 976/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk976<F: Float>(t14379: F, t470: F, t486: F, t494: F, t391: F, t79: F, t499: F, t493: F, t13949: F, t4204: F, t4203: F, t1505: F, t4181: F) -> (F, F, F, F) {
    let t14380 = t14379 * t470;
    let t14381 = t486 * t14380;
    let t14383 = t494 * t494;
    let t14386 = F::new(1.0) / t391 / t14383 * t79;
    let t14387 = t14386 * t499;
    let t14388 = t493 * t14387;
    let t14390 = t4204 * t13949;
    let t14391 = t4203 * t14390;
    let t14393 = t4181 * t1505;
    (t14381, t14388, t14391, t14393)
}
