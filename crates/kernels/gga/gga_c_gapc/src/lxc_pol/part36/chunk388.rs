//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 388/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk388<F: Float>(t1971: F, t204: F, t1645: F, t676: F, t618: F, t623: F, t617: F, t1403: F, t203: F, t153: F, t181: F, t628: F, t655: F, t683: F, t1552: F, t197: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1972 = t1971 * t204;
    let t1975 = t1645 * M_PI;
    let t1976 = t1975 * t676;
    let t1979 = t618 * t623;
    let t1980 = t617 * t1979;
    let t1983 = t203 * t1403;
    let t1984 = t153 * t1983;
    let t1985 = t181 * t1984;
    let t1988 = t628 * t655;
    let t1991 = t617 * t683;
    let t1994 = t203 * t1552;
    let t1995 = t618 * t1994;
    let t1996 = t197 * t1995;
    (t1972, t1975, t1976, t1979, t1980, t1983, t1985, t1988, t1991, t1996)
}
