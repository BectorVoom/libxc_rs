//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 578/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk578<F: Float>(t10122: F, t447: F, t1064: F, t3340: F, t535: F, t3347: F, t6305: F, t7930: F, t888: F, t2268: F, t2349: F, t2765: F) -> (F, F, F, F, F, F) {
    let t10123 = t10122 * t447;
    let t10124 = t1064 * t10123;
    let t10127 = t535 * t3340;
    let t10131 = F::cast_from(0.85365019907028448797e-1_f64) * t6305 * t3347;
    let t10132 = t7930 * t888;
    let t10134 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t10132;
    let t10135 = t2765 * t2349;
    (t10123, t10124, t10127, t10131, t10134, t10135)
}
