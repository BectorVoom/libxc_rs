//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 561/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk561<F: Float>(t701: F, t9603: F, t2580: F, t3270: F, t702: F, t3236: F, t779: F, t1987: F, t3276: F, t3248: F, t731: F, t3240: F) -> (F, F, F, F, F, F, F) {
    let t9604 = t9603 * t701;
    let t9605 = t2580 * t9604;
    let t9608 = t3270 * t702;
    let t9611 = t779 * t3236;
    let t9614 = t3276 * t1987;
    let t9618 = F::cast_from(0.85450291446024714264e-3_f64) * t731 * t3248;
    let t9620 = F::cast_from(0.85450291446024714264e-3_f64) * t731 * t3240;
    (t9604, t9605, t9608, t9611, t9614, t9618, t9620)
}
