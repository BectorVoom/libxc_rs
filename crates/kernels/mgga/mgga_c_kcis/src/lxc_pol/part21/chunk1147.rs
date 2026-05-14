//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1147/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1147<F: Float>(t2845: F, t4781: F, t4939: F, t1020: F, t4801: F, t92701: F, t1092: F, t14629: F, t27763: F, t3245: F, t8054: F, t1774: F, t303: F, t3170: F, t1014: F, t27928: F) -> (F, F, F, F, F, F) {
    let t95985 = t4939 * t4781 * t2845;
    let t95989 = t1020 * t92701 * t4801;
    let t95992 = t1092 * t27763 * t14629;
    let t96000 = t3245 * t8054;
    let t96003 = t303 * t3170 * t1774;
    let t96005 = t1014 * t27928;
    (t95985, t95989, t95992, t96000, t96003, t96005)
}
