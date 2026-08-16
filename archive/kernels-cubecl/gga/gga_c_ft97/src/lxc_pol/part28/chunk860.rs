//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 860/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk860<F: Float>(t1871: F, t7165: F, t986: F, t1339: F, t452: F, t6454: F, t26166: F, t6547: F, t11490: F, t34368: F, t83: F, t34544: F) -> (F, F, F, F, F, F) {
    let t34718 = t1871 * t986 * t7165;
    let t34722 = t452 * t1339 * t6454;
    let t34725 = t26166 * t6547;
    let t34726 = t11490 * t34725;
    let t34729 = t83 * t34368;
    let t34732 = t83 * t34544;
    (t34718, t34722, t34725, t34726, t34729, t34732)
}
