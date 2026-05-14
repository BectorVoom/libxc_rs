//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 904/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk904<F: Float>(t34074: F, t8477: F, t1892: F, t3140: F, t1501: F, t1936: F, t11120: F, t11239: F, t3268: F, t4147: F, t8594: F, t8598: F, t9593: F, t1450: F, t211: F, t9644: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t34075 = t8477 * t34074;
    let t34230 = t1892 * t3140;
    let t34231 = t8477 * t34230;
    let t34258 = t1501 * t1936;
    let t36865 = t11239 * t11120;
    let t36870 = t11239 * t3268;
    let t36970 = t4147 * t8594;
    let t37110 = t9593 * t8598;
    let t37956 = t8594 * t1450;
    let t37972 = t8598 * t4147;
    let t39643 = 1.0 / t9644 / t211;
    (t34075, t34230, t34231, t34258, t36865, t36870, t36970, t37110, t37956, t37972, t39643)
}
