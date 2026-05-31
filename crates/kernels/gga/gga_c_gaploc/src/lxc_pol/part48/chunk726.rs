//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 726/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk726<F: Float>(t13629: F, t13665: F, t13696: F, t13716: F, t1052: F, t10800: F, t13567: F, t13569: F, t13573: F, t13577: F, t13580: F, t13581: F, t13584: F, t13587: F, t1960: F, t2969: F, t331: F, t3511: F, t748: F) -> (F, F) {
    let t13718 = t13629 + t13665 + t13696 + t13716;
    let t13720 = -F::cast_from(2.0_f64) * t1052 * t10800 + t13567 * t331 + F::cast_from(4.0_f64) * t13581 * t1960 - t13718 * t748 - F::cast_from(2.0_f64) * t2969 * t3511 - t13569 + t13573 + t13577 - t13580 - t13584 + t13587;
    (t13718, t13720)
}
