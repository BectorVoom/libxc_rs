//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 913/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk913<F: Float>(t1988: F, t8978: F, t1089: F, t1579: F, t2079: F, t2080: F, t31276: F, t8544: F, t7310: F, t8505: F, t30225: F, t542: F, t1588: F, t7605: F, t2327: F, t7610: F) -> (F, F, F, F, F, F, F) {
    let t35930 = t1988 * t8978;
    let t35934 = t2079 * t1089 * t1579 * t2080;
    let t35936 = t31276 * t8544;
    let t35938 = t7310 * t8505;
    let t35949 = t30225 * t542;
    let t35951 = t7605 * t1588;
    let t35955 = t7610 * t2327;
    (t35930, t35934, t35936, t35938, t35949, t35951, t35955)
}
