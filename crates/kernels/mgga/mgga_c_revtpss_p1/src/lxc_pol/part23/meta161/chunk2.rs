//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 985/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk985<F: Float>(t2847: F, t2848: F, t4571: F, t4576: F, t4581: F, t4585: F, t291: F, t1596: F, t914: F) -> (F, F, F) {
    let t4587 = t2847 + F::cast_from(0.5936111111111111111e-2_f64) * t2848 + F::cast_from(0.5936111111111111111e-2_f64) * t4571 - F::cast_from(0.11872222222222222222e-1_f64) * t4576 + F::cast_from(0.35616666666666666666e-1_f64) * t4581 - F::cast_from(0.17808333333333333333e-1_f64) * t4585;
    let t4589 = F::new(0.621814e-1) * t4587 * t291;
    let t4590 = t1596 * t914;
    (t4587, t4589, t4590)
}
