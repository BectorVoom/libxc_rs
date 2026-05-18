//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 512/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk512<F: Float>(t2980: F, t2983: F, t129: F, t1875: F, t1877: F, t197: F, t1979: F, t1022: F, t122: F, t632: F) -> (F, F, F, F, F, F, F) {
    let t2984 = t2980 * t2983;
    let t2986 = t1875 * t129;
    let t2987 = t197 * t1877;
    let t2988 = t2986 * t2987;
    let t2990 = t197 * t1979;
    let t2991 = t1022 * t2990;
    let t2993 = t632 * t122;
    (t2984, t2986, t2987, t2988, t2990, t2991, t2993)
}
