//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1904/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1904(t102218: f64, t25878: f64, t2470: f64, t28844: f64, t7284: f64, t26292: f64, t27884: f64, t1904: f64, t26354: f64, t689: f64, t26271: f64, t27899: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102293 = t25878 * t102218;
    let t102295 = t28844 * t2470;
    let t102296 = t7284 * t102295;
    let t102298 = t27884 * t26292;
    let t102306 = 0.10975748638225852664e-1_f64 * t689 * t26354 * t1904;
    let t102309 = 0.14456046980341999104e-1_f64 * t27899 * t26271;
    (t102293, t102295, t102296, t102298, t102306, t102309)
}
