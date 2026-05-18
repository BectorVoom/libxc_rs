//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1151/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1151<F: Float>(t8319: F, t8392: F, t10044: F, t8467: F, t10047: F, t8456: F, t2344: F, t3849: F, t10122: F, t8428: F, t926: F, t10076: F, t8435: F) -> (F, F, F, F, F, F) {
    let t27151 = t8319 * t8392;
    let t27153 = t10044 * t8467;
    let t27155 = t10047 * t8456;
    let t27175 = t3849 * t2344;
    let t27178 = t8428 * t926 * t10122;
    let t27181 = t8435 * t926 * t10076;
    (t27151, t27153, t27155, t27175, t27178, t27181)
}
