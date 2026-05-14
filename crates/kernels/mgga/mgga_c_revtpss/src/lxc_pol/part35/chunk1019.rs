//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1019/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1019<F: Float>(t2453: F, t3908: F, t8086: F, t102218: F, t25878: F, t2470: F, t28844: F, t7284: F, t26292: F, t27884: F, t2435: F, t8099: F, t25904: F, t102100: F, t26069: F, t25899: F) -> (F, F, F, F, F, F, F, F) {
    let t102266 = t2453 * t8086 * t3908;
    let t102293 = t25878 * t102218;
    let t102295 = t28844 * t2470;
    let t102296 = t7284 * t102295;
    let t102298 = t27884 * t26292;
    let t102315 = t8099 * t2435;
    let t102316 = t25904 * t102315;
    let t102364 = t26069 * t102100;
    let t102378 = t25899 * t102315;
    (t102266, t102293, t102295, t102296, t102298, t102316, t102364, t102378)
}
