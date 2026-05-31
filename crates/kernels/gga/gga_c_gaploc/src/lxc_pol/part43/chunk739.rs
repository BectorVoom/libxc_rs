//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 739/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk739<F: Float>(t1397: F, t4390: F, t1238: F, t4072: F, t4081: F, t92: F, t153: F, t155: F, t4080: F, t121: F, t4524: F, t169: F, t4529: F) -> (F, F, F, F, F, F) {
    let t18067 = t1397 * t4390;
    let t18089 = F::cast_from(1.0_f64) / t4072 / t1238;
    let t18091 = t18089 * t92 * t4081;
    let t18096 = t153 / t4080 / t155;
    let t18310 = t121 * t4524;
    let t18313 = t169 * t4529;
    (t18067, t18089, t18091, t18096, t18310, t18313)
}
