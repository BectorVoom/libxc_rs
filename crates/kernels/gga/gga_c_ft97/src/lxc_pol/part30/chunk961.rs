//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 961/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk961<F: Float>(t1466: F, t34021: F, t681: F, t25488: F, t7581: F, t34253: F, t34277: F, t34329: F, t33966: F, t683: F, t2399: F, t7586: F) -> (F, F, F, F, F, F, F) {
    let t142611 = t1466 * t681 * t34021;
    let t142613 = t7581 * t25488;
    let t142618 = t1466 * t681 * t34253;
    let t142636 = t1466 * t681 * t34277;
    let t142647 = t1466 * t681 * t34329;
    let t142653 = t683 * t33966;
    let t142662 = F::new(4.0) / F::new(27.0) * t1466 * t2399 * t7586;
    (t142611, t142613, t142618, t142636, t142647, t142653, t142662)
}
