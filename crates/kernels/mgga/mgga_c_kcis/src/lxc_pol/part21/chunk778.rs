//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 778/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk778<F: Float>(t3025: F, t3034: F, t110: F, t1263: F, t1251: F, t25: F, t3612: F, t3483: F, t68: F) -> (F, F, F, F) {
    let t10974 = t3025 * t3034;
    let t10989 = t110 * t1263;
    let t10990 = t1251 * t10989;
    let t10992 = t25 * t3612;
    let t10993 = t1251 * t10992;
    let t10995 = t3483 * t68;
    (t10974, t10990, t10993, t10995)
}
