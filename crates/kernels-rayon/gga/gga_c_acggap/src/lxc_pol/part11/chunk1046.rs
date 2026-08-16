//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1046/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1046(t30543: f64, t8515: f64, t30398: f64, t30416: f64, t10146: f64, t420: f64, t576: f64, t1083: f64, t137: f64, t4257: f64, t30444: f64, t30365: f64, t30369: f64, t30375: f64, t30387: f64, t30397: f64, t30406: f64, t30412: f64, t30422: f64, t30429: f64, t30448: f64, t30452: f64, t30457: f64, t30459: f64) -> f64 {
    let t34361 = t30543 * t8515;
    let t34362 = 0.12862205435420921092e-1_f64 * t34361;
    let t34364 = 35.0_f64 / 216.0_f64 * t30398;
    let t34366 = 0.25158473831683321654e-2_f64 * t30416;
    let t34368 = t576 * t420 * t10146;
    let t34369 = t1083 * t137;
    let t34371 = t34368 * t34369 * t4257;
    let t34373 = 0.15724046144802076034e-2_f64 * t30444;
    let t34378 = -0.85748036236139473944e-3_f64 * t30365 + 0.20965394859736101378e-2_f64 * t30369 + 0.12579236915841660827e-2_f64 * t30375 - t34362 + 11.0_f64 / 384.0_f64 * t30387 - t30397 + t34364 - t30406 + 0.62896184579208304134e-2_f64 * t30412 - t34366 + t30422 + 0.183375e0_f64 * t34371 + t30429 - t34373 - 0.64311027177104605458e-3_f64 * t30448 + 0.62896184579208304136e-3_f64 * t30452 - 0.90035438047946447642e-2_f64 * t30457 + 0.42874018118069736972e-3_f64 * t30459;
    t34378
}
