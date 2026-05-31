//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 886/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk886<F: Float>(t1016: F, t10624: F, t1382: F, t11969: F, t2592: F, t2798: F, t2801: F, t33959: F, t32100: F, t10301: F, t8045: F, t11714: F, t7324: F) -> (F, F, F, F, F, F, F) {
    let t45123 = F::cast_from(4.0_f64) * t1382 * t1016 * t10624;
    let t45124 = t2592 * t11969;
    let t45126 = F::cast_from(2.0_f64) * t2798 * t10624;
    let t45130 = F::cast_from(4.0_f64) * t33959 * t2801;
    let t45132 = F::cast_from(2.0_f64) * t32100 * t1016;
    let t45134 = F::cast_from(4.0_f64) * t8045 * t10301;
    let t45141 = F::cast_from(4.0_f64) * t7324 * t11714;
    (t45123, t45124, t45126, t45130, t45132, t45134, t45141)
}
