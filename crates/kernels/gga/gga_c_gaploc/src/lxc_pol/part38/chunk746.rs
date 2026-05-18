//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 746/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk746<F: Float>(t3338: F, t447: F, t2366: F, t2754: F, t874: F, t6508: F, t2293: F, t986: F, t197: F, t10215: F, t158: F, t475: F) -> (F, F, F, F, F, F, F, F) {
    let t31557 = t3338 * t447;
    let t31558 = t2366 * t31557;
    let t31585 = t2754 * t874;
    let t31586 = t6508 * t31585;
    let t31590 = t986 * t2293;
    let t31591 = t6508 * t31590;
    let t31730 = t197 * t3338;
    let t31740 = t158 * t10215;
    let t31747 = t3338 * t475;
    (t31557, t31558, t31585, t31586, t31591, t31730, t31740, t31747)
}
