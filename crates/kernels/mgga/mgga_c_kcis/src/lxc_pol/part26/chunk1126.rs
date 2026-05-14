//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1126/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1126<F: Float>(t98794: F, t3245: F, t8171: F, t8179: F, t12147: F, t28550: F, t7908: F, t98487: F, t16937: F, t28488: F, t2237: F, t98537: F, t1014: F, t28476: F, t28426: F, t7895: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t98795 = 0.14739506172839506172e-2 * t98794;
    let t98804 = t3245 * t8171;
    let t98806 = t3245 * t8179;
    let t98813 = 0.15445601851851851852e-3 * t7908 * t12147 * t28550;
    let t98815 = 0.15445601851851851852e-3 * t7908 * t98487;
    let t98818 = 0.30891203703703703704e-3 * t7908 * t16937 * t28488;
    let t98820 = 0.46336805555555555556e-3 * t2237 * t98537;
    let t98822 = t1014 * t28476;
    let t98823 = 0.88437037037037037034e-2 * t98822;
    let t98825 = 0.46336805555555555556e-3 * t7895 * t28426;
    (t98795, t98804, t98806, t98813, t98815, t98818, t98820, t98822, t98823, t98825)
}
