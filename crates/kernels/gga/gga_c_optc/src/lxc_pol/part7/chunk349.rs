//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 349/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk349<F: Float>(t1128: F, t310: F, t448: F, t309: F, t447: F, t441: F) -> (F, F, F, F) {
    let t1129 = t310 * t1128;
    let t1131 = F::cast_from(0.18110753103726578864e-2_f64) * t448 * t1129;
    let t1132 = t447 * t309;
    let t1133 = t441 * t1132;
    (t1129, t1131, t1132, t1133)
}
