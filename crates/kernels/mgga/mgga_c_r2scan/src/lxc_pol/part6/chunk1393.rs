//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1393/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1393<F: Float>(t21155: F, t21159: F, t21161: F, t21164: F, t21168: F, t21173: F, t21176: F, t21177: F, t21179: F, t21183: F, t21186: F, t21191: F, t21195: F, t21200: F, t406: F, t7733: F) -> (F, F) {
    let t26415 = t21155 - t21159 + 0.24012257405919999999e-1 * t21161 + 0.48024514811839999998e-1 * t21164 - 0.10805515832664e0 * t21168 - t21173 + t21176 + 0.21687162600603479685e-1 * t21177 + 0.65061487801810439053e-1 * t21179 + t21183 + 0.20323535679999999999e-1 * t21186 + t21191 + t21195 - t21200;
    let t26420 = t406 * t7733;
    (t26415, t26420)
}
