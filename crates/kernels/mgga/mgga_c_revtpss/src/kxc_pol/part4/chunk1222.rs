//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1222/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1222<F: Float>(t1248: F, t1287: F, t5230: F, t1284: F, t1811: F, t1209: F, t13392: F, t5268: F, t1042: F, t1263: F, t3362: F, t15936: F, t3172: F, t5298: F, t3711: F, t1469: F, t3568: F) -> (F, F, F, F, F, F) {
    let t17188 = t5230 * t1248 * t1287;
    let t17191 = t1284 * t1811;
    let t17192 = t1209 * t17191;
    let t17198 = t5268 * t13392;
    let t17199 = t1042 * t17198;
    let t17202 = t1263 * t3362;
    let t17203 = t17202 * t15936;
    let t17204 = t1042 * t17203;
    let t17209 = t3172 * t5298;
    let t17211 = 0.19055119163586549765e-3 * t3711 * t17209;
    let t17212 = t1469 * t3568;
    (t17188, t17192, t17199, t17204, t17211, t17212)
}
