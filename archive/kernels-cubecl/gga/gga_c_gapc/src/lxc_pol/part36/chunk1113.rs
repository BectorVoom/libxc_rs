//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1113/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1113<F: Float>(t1038: F, t2619: F, t297: F, t33722: F, t7371: F, t11745: F, t18331: F, t11387: F, t7204: F, t7557: F, t11483: F, t11749: F, t2787: F) -> (F, F, F, F) {
    let t33726 = t2619 * t33722 * t1038 * t297 * t7371;
    let t33728 = t18331 * t11745;
    let t33731 = t7204 * t11387 * t7557;
    let t33734 = t2787 * t11483 * t11749;
    (t33726, t33728, t33731, t33734)
}
