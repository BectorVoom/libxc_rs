//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1043/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1043(t1025: f64, t1046: f64, t1935: f64, t30813: f64, t30817: f64, t30821: f64, t30824: f64, t30829: f64, t30833: f64, t30837: f64, t30840: f64, t378: f64, t6723: f64, t6730: f64, t6742: f64, t8384: f64) -> f64 {
    let t30843 = -0.32298204875312312685e-2_f64 * t6723 * t8384 + t30813 + 0.40372756094140390856e-3_f64 * t6730 * t8384 - 0.40372756094140390856e-3_f64 * t1935 * t30817 + 0.40372756094140390856e-3_f64 * t6742 * t30821 + t30824 * t378 / 1536.0_f64 + t30829 * t1025 / 1536.0_f64 - t30833 * t378 / 288.0_f64 + t30837 + t30840 * t1046 / 2304.0_f64;
    t30843
}
