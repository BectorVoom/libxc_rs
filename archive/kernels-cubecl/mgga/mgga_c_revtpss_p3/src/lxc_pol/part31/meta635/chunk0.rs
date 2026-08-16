//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2089/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2089<F: Float>(t16060: F, t7111: F, t25539: F, t4924: F, t16219: F, t139: F, t27526: F, t3252: F, t4574: F, t1014: F, t4579: F, t1035: F, t27543: F) -> (F, F, F, F, F, F) {
    let t100359 = t7111 * t16060 / F::cast_from(432.0_f64);
    let t100363 = t25539 * t4924 / F::cast_from(162.0_f64);
    let t100365 = t7111 * t16219;
    let t100370 = t27526 * t139 * t3252 * t4574 / F::cast_from(324.0_f64);
    let t100398 = t27526 * t139 * t1014 * t4579 / F::cast_from(216.0_f64);
    let t100431 = t1035 * t27543;
    (t100359, t100363, t100365, t100370, t100398, t100431)
}
