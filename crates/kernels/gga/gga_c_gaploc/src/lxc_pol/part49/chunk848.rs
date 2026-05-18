//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 848/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk848<F: Float>(t191: F, t4529: F, t107: F, t31730: F, t544: F, t6540: F, t986: F, t2299: F, t2754: F, t3394: F, t4130: F, t10417: F, t1397: F) -> (F, F, F, F, F, F) {
    let t34507 = t191 * t4529;
    let t34558 = t544 * t31730 * t107;
    let t34600 = t6540 * t986;
    let t34604 = t2299 * t2754;
    let t34688 = t4130 * t3394;
    let t34777 = t1397 * t10417;
    (t34507, t34558, t34600, t34604, t34688, t34777)
}
