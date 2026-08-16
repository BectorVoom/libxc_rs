//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 744/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk744<F: Float>(t2365: F, t4325: F, t7025: F, t1415: F, t1420: F, t2317: F, t900: F) -> (F, F, F, F) {
    let t7026 = t2365 * t4325;
    let t7027 = t7025 * t7026;
    let t7029 = t1415 * t1420;
    let t7030 = t900 * t2317;
    (t7026, t7027, t7029, t7030)
}
