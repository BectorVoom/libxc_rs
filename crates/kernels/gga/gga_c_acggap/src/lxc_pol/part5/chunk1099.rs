//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1099/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1099<F: Float>(t310: F, t6415: F, t1915: F, t3896: F, t15115: F, t557: F, t1658: F, t16986: F, t6472: F, t1814: F, t441: F) -> (F, F, F, F, F, F) {
    let t19704 = t310 * t6415;
    let t19706 = t3896 * t1915;
    let t19708 = t15115 * t557;
    let t19711 = t1658 * t1658;
    let t19716 = t16986 * t6472;
    let t19718 = t441 * t1814;
    (t19704, t19706, t19708, t19711, t19716, t19718)
}
