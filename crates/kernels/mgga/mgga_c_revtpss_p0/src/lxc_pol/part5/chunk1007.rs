//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1007/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1007<F: Float>(t785: F, t860: F, t780: F, t2439: F, t781: F, t9292: F, t861: F, t867: F, t786: F, t2410: F, t261: F, t262: F, t775: F) -> (F, F, F, F, F) {
    let t11028 = t785 * t860;
    let t11029 = t11028 * t780;
    let t11030 = t2439 * t11029;
    let t11040 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t781;
    let t11043 = t861 * t867;
    let t11044 = t786 * t11043;
    let t11064 = F::new(1.0) / t2410 / t261;
    let t11088 = t262 * t775;
    (t11030, t11040, t11044, t11064, t11088)
}
