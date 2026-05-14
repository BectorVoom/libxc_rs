//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 333/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk333<F: Float>(t1983: F, t557: F, t303: F, t1471: F, t1472: F, t1650: F, t1477: F, t1897: F) -> (F, F, F, F) {
    let t1984 = t1983 * t557;
    let t1985 = t303 * t1984;
    let t1988 = t1471 * t1472 * t1650;
    let t1991 = t1477 * t1897;
    (t1984, t1985, t1988, t1991)
}
