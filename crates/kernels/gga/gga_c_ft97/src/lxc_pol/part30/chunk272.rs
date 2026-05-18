//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 272/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk272<F: Float>(t1614: F, t213: F, t1109: F, t3762: F, t709: F, t1701: F, t1127: F, t25: F, t224: F, t226: F) -> (F, F, F, F, F, F) {
    let t3775 = t1614 * t213;
    let t3776 = t3775 * t1109;
    let t3777 = t3776 * t3762;
    let t3780 = t213 * t1109;
    let t3781 = t3780 * t709;
    let t3782 = t1701 * t3781;
    let t3785 = t1127 * t25;
    let t3786 = t3785 * t3762;
    let t3789 = t224 * t226;
    (t3777, t3780, t3781, t3782, t3786, t3789)
}
