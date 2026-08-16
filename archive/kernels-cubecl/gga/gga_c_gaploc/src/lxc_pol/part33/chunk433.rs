//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 433/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk433<F: Float>(t2066: F, t774: F, t769: F, t779: F, t1836: F, t531: F, t1865: F, t808: F, t568: F, t836: F, t321: F) -> (F, F, F, F, F, F) {
    let t2067 = t2066 * t774;
    let t2070 = t769 * t779;
    let t2073 = t531 * t1836;
    let t2076 = t808 * t1865;
    let t2077 = t568 * t2076;
    let t2080 = t836 * t1865;
    let t2081 = t568 * t2080;
    let t2084 = t321 * t321;
    (t2067, t2070, t2073, t2077, t2081, t2084)
}
