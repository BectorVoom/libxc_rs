//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 773/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk773<F: Float>(t1397: F, t8410: F, t1: F, t106: F, t4524: F, t544: F, t191: F, t4529: F, t6540: F, t986: F, t2299: F, t2754: F) -> (F, F, F, F, F) {
    let t34471 = t1397 * t8410;
    let t34506 = t544 * t4524 * t1 * t106;
    let t34507 = t191 * t4529;
    let t34600 = t6540 * t986;
    let t34604 = t2299 * t2754;
    (t34471, t34506, t34507, t34600, t34604)
}
