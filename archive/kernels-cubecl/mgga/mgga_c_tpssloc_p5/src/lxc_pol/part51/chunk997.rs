//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 997/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk997<F: Float>(t25241: F, t6646: F, t1888: F, t23110: F, t7524: F, t23185: F, t234: F, t6604: F, t1484: F, t252: F, t776: F, t25038: F) -> (F, F, F, F, F) {
    let t25242 = t6646 * t25241;
    let t25243 = t1888 * t25242;
    let t25245 = t23110 * t7524;
    let t25246 = t23185 * t25245;
    let t25248 = t6604 * t234;
    let t25249 = t252 * t1484;
    let t25250 = t25249 * t776;
    let t25251 = t25248 * t25250;
    let t25252 = t25038 * t25251;
    (t25243, t25246, t25248, t25249, t25252)
}
