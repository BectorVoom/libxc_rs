//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1142/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1142<F: Float>(t10969: F, t25590: F, t1332: F, t59565: F, t4551: F, t91496: F, t1286: F, t29460: F, t376: F, t25545: F, t6414: F, t29741: F, t5498: F, t102270: F, t102312: F, t1310: F, t16110: F, t2: F, t22883: F, t25528: F, t25533: F, t26: F, t26493: F, t28: F, t38921: F, t4: F, t4436: F, t497: F, t5501: F, t5502: F, t75950: F, t948: F) -> (F, F, F, F) {
    let t116145 = t10969 * t25590;
    let t116155 = t59565 * t1332;
    let t116157 = t91496 * t4551;
    let t116160 = t1286 * t376 * t29460;
    let t116167 = t6414 * t25545;
    let t116169 = t29741 * t5498;
    let t116176 = -4.0 / 81.0 * t102270 - 2.0 * t948 * t26493 + 8.0 * t116145 + t1286 * t28 * t22883 * t497 * t4436 - 2.0 / 3.0 * t1286 * t28 * t25528 * t25533 - 2.0 * t116155 + 4.0 * t116157 - t116160 / 18.0 + t75950 * t2 * t4 * t26 * t1310 / 6.0 - t116167 / 9.0 - t116169 / 18.0 - 4.0 * t5501 * t38921 * t5502 * t16110 - 8.0 / 27.0 * t102312;
    (t116145, t116155, t116157, t116176)
}
