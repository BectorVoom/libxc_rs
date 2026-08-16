//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 972/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk972<F: Float>(t231: F, t33829: F, t33840: F, t6308: F, t681: F, t1486: F, t33852: F, t33954: F, t2347: F, t7611: F, t33855: F, t2360: F, t7584: F) -> (F, F, F, F, F, F, F) {
    let t143163 = t231 * t33829;
    let t143177 = t6308 * t681 * t33840;
    let t143180 = t1486 * t681 * t33852;
    let t143187 = t1486 * t681 * t33954;
    let t143193 = t7611 * t2347;
    let t143204 = t1486 * t681 * t33855;
    let t143217 = t7584 * t2360;
    (t143163, t143177, t143180, t143187, t143193, t143204, t143217)
}
