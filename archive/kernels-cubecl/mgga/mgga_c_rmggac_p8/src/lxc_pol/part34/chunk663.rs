//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 663/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk663<F: Float>(t2123: F, t570: F, t2367: F, t321: F, t118: F, t25809: F, t558: F, t35959: F, t3839: F, t3851: F, t22: F, t235: F, t34812: F) -> (F, F, F, F, F, F, F) {
    let t41063 = t2123 * t570;
    let t41091 = t2367 * t321;
    let t41116 = t118 * t25809;
    let t41122 = t2123 * t558;
    let t41400 = t3839 * t35959;
    let t41407 = t3851 * t35959;
    let t41738 = t235 * t34812 * t22;
    (t41063, t41091, t41116, t41122, t41400, t41407, t41738)
}
