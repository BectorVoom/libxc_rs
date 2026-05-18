//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1046/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1046<F: Float>(t30543: F, t8515: F, t30398: F, t30416: F, t10146: F, t420: F, t576: F, t1083: F, t137: F, t4257: F, t30444: F, t30365: F, t30369: F, t30375: F, t30387: F, t30397: F, t30406: F, t30412: F, t30422: F, t30429: F, t30448: F, t30452: F, t30457: F, t30459: F) -> F {
    let t34361 = t30543 * t8515;
    let t34362 = F::new(0.12862205435420921092e-1) * t34361;
    let t34364 = F::new(35.0) / F::new(216.0) * t30398;
    let t34366 = F::new(0.25158473831683321654e-2) * t30416;
    let t34368 = t576 * t420 * t10146;
    let t34369 = t1083 * t137;
    let t34371 = t34368 * t34369 * t4257;
    let t34373 = F::new(0.15724046144802076034e-2) * t30444;
    let t34378 = -F::new(0.85748036236139473944e-3) * t30365 + F::new(0.20965394859736101378e-2) * t30369 + F::new(0.12579236915841660827e-2) * t30375 - t34362 + F::new(11.0) / F::new(384.0) * t30387 - t30397 + t34364 - t30406 + F::new(0.62896184579208304134e-2) * t30412 - t34366 + t30422 + F::new(0.183375e0) * t34371 + t30429 - t34373 - F::new(0.64311027177104605458e-3) * t30448 + F::new(0.62896184579208304136e-3) * t30452 - F::new(0.90035438047946447642e-2) * t30457 + F::new(0.42874018118069736972e-3) * t30459;
    t34378
}
