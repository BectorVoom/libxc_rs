//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1904/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1904<F: Float>(t102218: F, t25878: F, t2470: F, t28844: F, t7284: F, t26292: F, t27884: F, t1904: F, t26354: F, t689: F, t26271: F, t27899: F) -> (F, F, F, F, F, F) {
    let t102293 = t25878 * t102218;
    let t102295 = t28844 * t2470;
    let t102296 = t7284 * t102295;
    let t102298 = t27884 * t26292;
    let t102306 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t26354 * t1904;
    let t102309 = F::cast_from(0.14456046980341999104e-1_f64) * t27899 * t26271;
    (t102293, t102295, t102296, t102298, t102306, t102309)
}
