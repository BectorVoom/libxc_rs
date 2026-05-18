//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1195/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1195<F: Float>(t12210: F, t37346: F, t11506: F, t38697: F, t10626: F, t12056: F, t3275: F, t11458: F, t40282: F, t38715: F, t40394: F, t11455: F) -> (F, F, F, F, F, F) {
    let t41256 = F::new(3.0) / F::new(4.0) * t37346 * t12210;
    let t41258 = F::new(3.0) / F::new(4.0) * t11506 * t38697;
    let t41261 = t3275 * t12056 * t10626 / F::new(2.0);
    let t41263 = F::new(3.0) / F::new(2.0) * t40282 * t11458;
    let t41265 = F::new(3.0) / F::new(2.0) * t40394 * t38715;
    let t41270 = F::new(15.0) / F::new(8.0) * t40282 * t11455;
    (t41256, t41258, t41261, t41263, t41265, t41270)
}
