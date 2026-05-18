//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 644/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk644<F: Float>(t3337: F, t5091: F, t1196: F, t1809: F, t1195: F, t1812: F, t1187: F, t3438: F, t4823: F) -> (F, F, F, F, F) {
    let t5092 = t3337 * t5091;
    let t5094 = t1809 * t1196;
    let t5096 = t1195 * t1812;
    let t5097 = t1187 * t5096;
    let t5099 = t3438 * t4823;
    (t5092, t5094, t5096, t5097, t5099)
}
