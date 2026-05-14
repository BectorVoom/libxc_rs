//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1050/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1050<F: Float>(t1358: F, t2339: F, t25761: F, t6536: F, t7888: F, t2754: F, t874: F) -> (F, F, F) {
    let t31581 = 0.18970004423784099733e-1 * t1358 * t25761 * t2339;
    let t31584 = 0.18970004423784099733e-1 * t1358 * t7888 * t6536;
    let t31585 = t2754 * t874;
    (t31581, t31584, t31585)
}
