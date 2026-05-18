//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1015/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1015<F: Float>(t1070: F, t2938: F, t1276: F, t2983: F, t352: F, t12428: F, t3275: F, t3472: F, t12086: F, t3579: F, t12570: F, t3262: F, t3465: F) -> (F, F, F, F, F, F) {
    let t12598 = t1070 * t2938;
    let t12599 = t1276 * t12598;
    let t12683 = t352 * t2983;
    let t12720 = t3275 * t3472 * t12428;
    let t12721 = F::new(5.0) / F::new(16.0) * t12720;
    let t12722 = t3579 * t12086;
    let t12723 = t12722 / F::new(2.0);
    let t12725 = t3262 * t3465 * t12570;
    (t12598, t12599, t12683, t12721, t12723, t12725)
}
