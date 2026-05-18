//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 336/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk336<F: Float>(t1981: F, t509: F, t552: F, t557: F, t303: F, t1471: F, t1472: F, t1650: F, t1477: F, t1897: F, t542: F, t1482: F, t1961: F) -> (F, F, F, F, F, F, F, F) {
    let t1982 = t509 * t1981;
    let t1983 = t1982 * t552;
    let t1984 = t1983 * t557;
    let t1985 = t303 * t1984;
    let t1988 = t1471 * t1472 * t1650;
    let t1991 = t1477 * t1897;
    let t1992 = t542 * t1991;
    let t1995 = t1482 * t1961;
    (t1982, t1983, t1984, t1985, t1988, t1991, t1992, t1995)
}
