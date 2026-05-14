//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1024/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1024<F: Float>(t12321: F, t2011: F, t12234: F, t1385: F, t3751: F, t4992: F, t86: F, t3960: F, t5623: F, t1494: F, t5627: F, t1380: F, t167: F, t1650: F, t4007: F, t3977: F, t498: F) -> (F, F, F, F, F, F, F, F) {
    let t51613 = t12321 * t2011;
    let t51622 = t12234 * t1385;
    let t51692 = t86 * t4992 * t3751;
    let t51799 = t5623 * t3960;
    let t52073 = t1494 * t5627;
    let t52371 = t167 * t1380;
    let t52402 = t1650 * t4007;
    let t52460 = t3977 * t498;
    (t51613, t51622, t51692, t51799, t52073, t52371, t52402, t52460)
}
