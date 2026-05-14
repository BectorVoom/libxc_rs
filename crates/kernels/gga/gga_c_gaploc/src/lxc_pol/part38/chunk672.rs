//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 672/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk672<F: Float>(t6540: F, t986: F, t2299: F, t2754: F, t3394: F, t4130: F, t10417: F, t1397: F, t10241: F, t9448: F, t9439: F, t31557: F, t493: F, t1339: F, t31585: F, t20550: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t34600 = t6540 * t986;
    let t34604 = t2299 * t2754;
    let t34688 = t4130 * t3394;
    let t34777 = t1397 * t10417;
    let t34814 = t9448 * t10241;
    let t34818 = t9439 * t10241;
    let t34882 = t493 * t31557;
    let t34890 = t1339 * t3394;
    let t35045 = t1339 * t31585;
    let t35091 = t4130 * t2754;
    let t35101 = t20550 * t10241;
    (t34600, t34604, t34688, t34777, t34814, t34818, t34882, t34890, t35045, t35091, t35101)
}
