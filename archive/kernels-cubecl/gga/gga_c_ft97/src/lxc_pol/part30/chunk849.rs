//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 849/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk849<F: Float>(t1131: F, t7553: F, t729: F, t762: F, t1168: F, t2568: F, t242: F, t2574: F, t265: F, t35353: F, t1456: F, t6852: F) -> (F, F, F, F, F, F, F) {
    let t35634 = t7553 * t1131;
    let t35636 = t729 * t762 * t35634;
    let t35639 = t7553 * t1168;
    let t35640 = t2568 * t35639;
    let t35641 = t242 * t35640;
    let t35645 = t2574 * t265 * t35353;
    let t35649 = t2574 * t1456 * t6852;
    (t35634, t35636, t35639, t35640, t35641, t35645, t35649)
}
