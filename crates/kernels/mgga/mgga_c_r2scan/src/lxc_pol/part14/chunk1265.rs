//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1265/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1265<F: Float>(t3579: F, t38723: F, t3275: F, t3472: F, t39178: F, t11325: F, t11518: F, t3262: F, t11189: F, t40289: F, t3465: F, t40667: F) -> (F, F, F, F, F) {
    let t42277 = t3579 * t38723 / F::new(2.0);
    let t42281 = F::new(5.0) / F::new(16.0) * t3275 * t3472 * t39178;
    let t42284 = F::new(15.0) / F::new(8.0) * t3262 * t11325 * t11518;
    let t42287 = F::new(45.0) / F::new(64.0) * t3275 * t11189 * t40289;
    let t42290 = F::new(3.0) / F::new(2.0) * t3275 * t3465 * t40667;
    (t42277, t42281, t42284, t42287, t42290)
}
