//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1240/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1240<F: Float>(t1542: F, t1928: F, t16744: F, t491: F, t990: F, t1494: F, t2242: F, t28361: F, t3728: F, t16937: F, t28442: F, t27369: F) -> (F, F, F, F, F, F, F) {
    let t98020 = t1542 * t1928;
    let t98025 = t16744 * t491 * t990;
    let t98034 = t2242 * t1494;
    let t98057 = t3728 * t28361;
    let t98058 = F::new(0.22109259259259259258e-2) * t98057;
    let t98072 = t16937 * t28442;
    let t98074 = F::new(0.20612155671296296296e-4) * t27369 * t98072;
    (t98020, t98025, t98034, t98057, t98058, t98072, t98074)
}
