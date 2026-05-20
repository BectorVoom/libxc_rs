//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1143/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1143<F: Float>(t2470: F, t28844: F, t7284: F, t26292: F, t27884: F, t2435: F, t8099: F, t25904: F, t102100: F, t26069: F, t25899: F, t2439: F) -> (F, F, F, F, F, F, F) {
    let t102295 = t28844 * t2470;
    let t102296 = t7284 * t102295;
    let t102298 = t27884 * t26292;
    let t102315 = t8099 * t2435;
    let t102316 = t25904 * t102315;
    let t102364 = t26069 * t102100;
    let t102378 = t25899 * t102315;
    let t102385 = t8099 * t2439;
    (t102295, t102296, t102298, t102316, t102364, t102378, t102385)
}
