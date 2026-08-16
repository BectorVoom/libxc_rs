//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1008/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1008<F: Float>(t4973: F, t835: F, t882: F, t18123: F, t319: F, t2857: F, t4965: F, t1091: F, t4181: F, t15312: F, t1248: F, t505: F) -> (F, F, F, F, F) {
    let t19606 = t835 * t882 * t4973;
    let t19610 = t835 * t319 * t18123;
    let t19614 = t2857 * t882 * t4965;
    let t19617 = t1091 * t4181;
    let t19618 = t15312 * t19617;
    let t19621 = t1248 * t505;
    (t19606, t19610, t19614, t19618, t19621)
}
