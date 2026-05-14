//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 820/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk820<F: Float>(t66934: F, t4930: F, t89: F, t9733: F, t5106: F, t8282: F, t5099: F, t5110: F, t5102: F, t1771: F, t5114: F, t5118: F, t41955: F, t4918: F, t5157: F, t8232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t66935 = 8.0 / 27.0 * t66934;
    let t66945 = t89 * t9733 * t4930;
    let t66946 = 4.0 / 27.0 * t66945;
    let t67078 = t8282 * t5106;
    let t67097 = t8282 * t5099;
    let t67103 = t8282 * t5110;
    let t67288 = t8282 * t5102;
    let t67329 = t1771 * t5114;
    let t67331 = t1771 * t5118;
    let t67420 = t89 * t41955 * t4918;
    let t67421 = 8.0 / 81.0 * t67420;
    let t67746 = t8232 * t5157;
    (t66935, t66945, t66946, t67078, t67097, t67103, t67288, t67329, t67331, t67420, t67421, t67746)
}
