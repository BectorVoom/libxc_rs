//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 999/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk999<F: Float>(t262: F, t41043: F, t2068: F, t36: F, t4928: F, t2073: F, t2079: F, t5249: F, t1587: F, t265: F, t2123: F, t551: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t41044 = t262 * t41043;
    let t41045 = t2068 * t41044;
    let t41047 = t36 * t4928;
    let t41048 = t262 * t41047;
    let t41049 = t2073 * t41048;
    let t41053 = t2079 * t262 * t36 * t5249;
    let t41055 = t265 * t1587;
    let t41056 = t262 * t41055;
    let t41057 = t2068 * t41056;
    let t41059 = t2123 * t551;
    (t41044, t41045, t41047, t41048, t41049, t41053, t41055, t41056, t41057, t41059)
}
