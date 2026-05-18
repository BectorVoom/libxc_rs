//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 873/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk873<F: Float>(t1017: F, t13428: F, t86: F, t4557: F, t3217: F, t330: F, t1646: F, t3219: F, t3210: F, t3200: F, t3203: F, t3228: F) -> (F, F, F, F) {
    let t13430 = t86 * t1017 * t13428;
    let t13431 = t13430 * t4557;
    let t13433 = t3217 * t330;
    let t13435 = t13433 * t1646 * t3219;
    let t13436 = t3210 * t13435;
    let t13437 = t3200 * t13436;
    let t13440 = t3203 * t1646 * t3228;
    (t13431, t13435, t13437, t13440)
}
