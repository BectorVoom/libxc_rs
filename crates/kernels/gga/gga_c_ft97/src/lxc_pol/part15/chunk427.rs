//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 427/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk427<F: Float>(t1212: F, t2: F, t4032: F, t4049: F, t1240: F, t870: F) -> (F, F, F, F) {
    let t4218 = t2 * t1212;
    let t4230 = t4032 / F::new(27.0);
    let t4235 = t4049 / F::new(9.0);
    let t4246 = t1240 * t870;
    (t4218, t4230, t4235, t4246)
}
