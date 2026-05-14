//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 357/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk357<F: Float>(t492: F, t5710: F, t83: F, t1334: F, t1882: F, t1332: F, t487: F, t379: F, t1909: F, t432: F) -> (F, F, F, F, F, F) {
    let t5711 = t5710 * t492;
    let t5712 = t83 * t5711;
    let t5716 = t1882 * t1334 / 9.0;
    let t5717 = t487 * t1332;
    let t5718 = t5717 * t379;
    let t5719 = t1909 * t5718;
    let t5722 = t1332 * t432;
    (t5712, t5716, t5717, t5718, t5719, t5722)
}
