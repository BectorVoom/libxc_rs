//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1025/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1025<F: Float>(t126: F, t17890: F, t276: F, t314: F, t442: F, t2206: F, t2250: F, t103: F, t2723: F, t1087: F, t2404: F, t1: F, t6852: F) -> (F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t22949 = t276 * t17890 * t126;
    let t22954 = t314 * pi * t442;
    let t22970 = t2250 * t2206;
    let t22971 = t22970 * t126;
    let t22973 = t2723 * t103;
    let t23104 = t1087 * t2404;
    let t23132 = t6852 * t1;
    (t22949, t22954, t22970, t22971, t22973, t23104, t23132)
}
