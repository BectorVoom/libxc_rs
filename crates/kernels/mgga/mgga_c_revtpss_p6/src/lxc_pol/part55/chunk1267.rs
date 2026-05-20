//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1267/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1267<F: Float>(t196: F, t197: F, t28686: F, t2035: F, t34270: F, t7313: F, t28021: F, t8698: F, t27833: F, t8715: F, t32735: F, t7898: F) -> (F, F, F, F, F) {
    let t128958 = t28686 * t196 * t197;
    let t128959 = t128958 * t2035;
    let t128960 = t34270 * t7313;
    let t128964 = t8698 * t28021;
    let t128965 = t27833 * t8715;
    let t128966 = t7898 * t32735;
    (t128959, t128960, t128964, t128965, t128966)
}
