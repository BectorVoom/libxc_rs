//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 676/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk676<F: Float>(t6856: F, t2250: F, t825: F, t126: F, t2723: F, t442: F, t2669: F, t2206: F, t768: F) -> (F, F, F, F, F, F) {
    let t6857 = t6856 * M_PI;
    let t6924 = t2250 * t825;
    let t6925 = t6924 * t126;
    let t6927 = t2723 * t442;
    let t6935 = t2669 * t442;
    let t6939 = t768 * t2206;
    (t6857, t6924, t6925, t6927, t6935, t6939)
}
