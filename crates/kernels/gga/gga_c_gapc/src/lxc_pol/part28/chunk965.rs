//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 965/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk965<F: Float>(t126: F, t22970: F, t103: F, t2723: F, t1087: F, t2404: F, t1: F, t6852: F, t640: F, t6939: F, t102: F, t2446: F, t2254: F, t830: F, t122: F, t6924: F) -> (F, F, F, F, F, F, F, F) {
    let t22971 = t22970 * t126;
    let t22973 = t2723 * t103;
    let t23104 = t1087 * t2404;
    let t23132 = t6852 * t1;
    let t23305 = t6939 * t640;
    let t23343 = t2446 * t102 * t2404;
    let t23466 = t830 * t2254;
    let t23523 = t6924 * t122;
    (t22971, t22973, t23104, t23132, t23305, t23343, t23466, t23523)
}
