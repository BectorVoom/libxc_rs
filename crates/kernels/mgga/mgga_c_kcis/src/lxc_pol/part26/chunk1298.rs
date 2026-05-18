//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1298/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1298<F: Float>(t1464: F, t1497: F, t58599: F, t7923: F, t1616: F, t7429: F, t1307: F, t22722: F, t6159: F, t1394: F, t20873: F, t27387: F) -> (F, F, F, F) {
    let t102205 = t1464 * t7923 * t58599 * t1497;
    let t102209 = t1616 * t7429;
    let t102221 = t6159 * t22722 * t1307;
    let t102237 = t1394 * t27387 * t20873;
    (t102205, t102209, t102221, t102237)
}
