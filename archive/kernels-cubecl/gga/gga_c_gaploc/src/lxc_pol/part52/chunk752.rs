//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 752/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk752<F: Float>(t191: F, t4529: F, t6540: F, t986: F, t2299: F, t2754: F, t3394: F, t4130: F, t10241: F, t9448: F, t9439: F, t1339: F) -> (F, F, F, F, F, F, F) {
    let t34507 = t191 * t4529;
    let t34600 = t6540 * t986;
    let t34604 = t2299 * t2754;
    let t34688 = t4130 * t3394;
    let t34814 = t9448 * t10241;
    let t34818 = t9439 * t10241;
    let t34890 = t1339 * t3394;
    (t34507, t34600, t34604, t34688, t34814, t34818, t34890)
}
