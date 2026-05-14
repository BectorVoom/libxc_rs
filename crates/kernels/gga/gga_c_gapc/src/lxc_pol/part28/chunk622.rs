//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 622/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk622<F: Float>(t126: F, t1458: F, t102: F, t567: F, t1593: F, t1798: F, t442: F, t1901: F, t22: F) -> (F, F, F, F, F) {
    let t4015 = t1458 * t126;
    let t4017 = t102 * t567;
    let t4018 = t1593 * t4017;
    let t4026 = t1798 * t442;
    let t4043 = 1.0 / t22 / t1901;
    (t4015, t4017, t4018, t4026, t4043)
}
