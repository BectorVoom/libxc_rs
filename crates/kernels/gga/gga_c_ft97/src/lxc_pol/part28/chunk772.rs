//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 772/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk772<F: Float>(t452: F, t5710: F, t6538: F, t7165: F, t979: F, t1871: F, t488: F, t110: F, t34384: F, t34379: F, t8411: F, t7288: F, t942: F, t7211: F, t986: F, t34482: F) -> (F, F, F, F, F, F, F, F) {
    let t34737 = t452 * t5710 * t6538;
    let t34740 = t7165 * t979;
    let t34742 = t1871 * t488 * t34740;
    let t34746 = t1871 * t110 * t34384;
    let t34750 = t8411 * t110 * t34379;
    let t34754 = t452 * t7288 * t942;
    let t34758 = t452 * t986 * t7211;
    let t34762 = t452 * t110 * t34482;
    (t34737, t34740, t34742, t34746, t34750, t34754, t34758, t34762)
}
