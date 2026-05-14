//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1192/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1192<F: Float>(t101603: F, t23050: F, t25893: F, t6495: F, t100190: F, t22958: F, t5674: F, t38477: F, t5675: F, t100356: F, t11064: F, t25982: F, t93506: F, t1647: F, t22952: F, t22953: F) -> (F, F, F, F, F, F) {
    let t101606 = t25893 * t101603 * t6495 * t23050;
    let t101609 = t5674 * t22958 * t100190;
    let t101611 = t38477 * t5675;
    let t101613 = t100356 * t101611 * t11064;
    let t101615 = t93506 * t25982;
    let t101616 = t101615 / 54.0;
    let t101619 = t22952 * t22953 * t6495 * t1647;
    (t101606, t101609, t101613, t101615, t101616, t101619)
}
