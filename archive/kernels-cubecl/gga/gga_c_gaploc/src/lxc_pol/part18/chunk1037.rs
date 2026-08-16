//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1037/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1037<F: Float>(t153: F, t155: F, t4080: F, t121: F, t4524: F, t169: F, t4529: F, t1406: F, t4780: F, t1535: F, t15478: F, t4324: F, t9448: F) -> (F, F, F, F, F, F) {
    let t18096 = t153 / t4080 / t155;
    let t18310 = t121 * t4524;
    let t18313 = t169 * t4529;
    let t18337 = t1406 * t4780;
    let t18362 = t1535 * t15478;
    let t18364 = t9448 * t4324;
    (t18096, t18310, t18313, t18337, t18362, t18364)
}
