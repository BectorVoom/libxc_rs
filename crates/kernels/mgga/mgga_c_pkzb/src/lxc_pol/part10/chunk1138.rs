//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1138/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1138<F: Float>(t149: F, t1773: F, t95: F, t5402: F, t579: F, t583: F, t1712: F, t5264: F, t1698: F, t1705: F, t1708: F, t2590: F, t5256: F, t1717: F, t5295: F, t1723: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16373 = t149 * t95 * t1773;
    let t16378 = t5402 * t579;
    let t16379 = t16378 * t583;
    let t16381 = t5264 * t1712;
    let t16388 = t1698 * t1705;
    let t16389 = t16388 * t1708;
    let t16399 = t2590 * t5256;
    let t16402 = t1717 * t5295;
    let t16403 = t16402 * t1723;
    (t16373, t16378, t16379, t16381, t16388, t16389, t16399, t16402, t16403)
}
