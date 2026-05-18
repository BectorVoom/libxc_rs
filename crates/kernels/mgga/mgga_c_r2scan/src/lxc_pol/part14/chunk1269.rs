//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1269/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1269<F: Float>(t39030: F, t40630: F, t40631: F, t12197: F, t1561: F, t3275: F, t3277: F, t10630: F, t12056: F, t3262: F, t3352: F, t41202: F) -> (F, F, F, F) {
    let t42330 = F::new(3.0) * t40630 * t39030 * t40631;
    let t42331 = t1561 * t12197;
    let t42334 = F::new(5.0) / F::new(8.0) * t3275 * t42331 * t3277;
    let t42339 = F::new(3.0) / F::new(4.0) * t3262 * t12056 * t10630;
    let t42344 = t3275 * t41202 * t3352 / F::new(2.0);
    (t42330, t42334, t42339, t42344)
}
