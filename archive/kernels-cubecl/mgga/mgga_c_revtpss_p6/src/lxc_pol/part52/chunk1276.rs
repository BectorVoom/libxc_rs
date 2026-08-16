//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1276/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1276<F: Float>(t1448: F, t25082: F, t28286: F, t34301: F, t196: F, t197: F, t28686: F, t2035: F, t34270: F, t7313: F, t28021: F, t8698: F) -> (F, F, F, F) {
    let t128945 = F::cast_from(6.0_f64) * t25082 * t28286 * t34301 * t1448;
    let t128958 = t28686 * t196 * t197;
    let t128959 = t128958 * t2035;
    let t128960 = t34270 * t7313;
    let t128964 = t8698 * t28021;
    (t128945, t128959, t128960, t128964)
}
