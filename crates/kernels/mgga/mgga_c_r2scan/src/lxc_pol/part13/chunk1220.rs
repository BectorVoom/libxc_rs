//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1220/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1220<F: Float>(t14656: F, t795: F, t3270: F, t3269: F, t23987: F, t3263: F, t3275: F, t38259: F, t38261: F, t38265: F, t38268: F, t38270: F, t40619: F, t40623: F, t40626: F, t40628: F, t40634: F, t40638: F, t40642: F, t40647: F) -> (F, F, F) {
    let t40648 = t14656 * t795;
    let t40649 = t3270 * t40648;
    let t40651 = t3269 * t40649 / F::new(2.0);
    let t40652 = t23987 * t795;
    let t40654 = t3275 * t3263 * t40652;
    let t40655 = t40619 - t40623 - t40626 - t40628 - t40634 + t40638 - F::new(0.30487649791575028314e-3) * t38259 + F::new(0.30487649791575028314e-3) * t38261 - t38265 - t38268 - t38270 + F::new(0.30487649791575028314e-3) * t40642 - t40647 + t40651 - t40654;
    (t40651, t40654, t40655)
}
