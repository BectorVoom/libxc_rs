//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1000/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1000<F: Float>(t447: F, t46849: F, t204: F, t2476: F, t40225: F, t38674: F, t544: F, t9287: F, t2365: F, t38272: F, t7025: F, t38770: F, t901: F) -> (F, F, F, F, F, F) {
    let t47953 = t46849 * t447;
    let t47955 = t2476 * t204 * t47953;
    let t47963 = F::cast_from(0.15337170381568299871e1_f64) * t40225;
    let t47964 = t544 * t38674;
    let t47965 = t47964 * t9287;
    let t47968 = t7025 * t2365 * t38272;
    let t47976 = t38770 * t901;
    (t47953, t47955, t47963, t47965, t47968, t47976)
}
