//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 928/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk928<F: Float>(t1609: F, t6109: F, t2926: F, t11299: F, t11144: F, t22688: F, t11341: F, t141: F, t22671: F, t905: F, t930: F, t11142: F) -> (F, F, F, F, F, F, F) {
    let t23466 = t6109 * t1609;
    let t23467 = t23466 * t2926;
    let t23469 = F::new(0.96491876992155210402e2) * t11299 * t23467;
    let t23470 = t11144 * t22688;
    let t23471 = t11341 * t23470;
    let t23472 = t141 * t23471;
    let t23474 = t905 * t22671;
    let t23475 = t930 * t23474;
    let t23476 = t141 * t23475;
    let t23478 = t11142 * t23470;
    (t23466, t23469, t23470, t23472, t23474, t23476, t23478)
}
