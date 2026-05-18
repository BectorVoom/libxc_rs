//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1023/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1023<F: Float>(t1446: F, t7614: F, t1988: F, t8978: F, t1089: F, t1579: F, t2079: F, t2080: F, t31276: F, t8544: F, t7310: F, t8505: F) -> (F, F, F, F, F) {
    let t35926 = t7614 * t1446;
    let t35930 = t1988 * t8978;
    let t35934 = t2079 * t1089 * t1579 * t2080;
    let t35936 = t31276 * t8544;
    let t35938 = t7310 * t8505;
    (t35926, t35930, t35934, t35936, t35938)
}
