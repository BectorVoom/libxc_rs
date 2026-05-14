//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 752/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk752<F: Float>(t17355: F, t605: F, t144: F, t1882: F, t4819: F, t4815: F, t3478: F, t925: F, t9144: F, t3483: F, t13220: F, t11593: F, t13040: F, t13042: F, t13049: F, t13062: F, t13075: F, t13084: F, t17195: F, t17200: F, t17204: F, t17208: F, t1901: F, t446: F) -> (F, F) {
    let t17356 = t605 * t17355;
    let t17357 = t144 * t17356;
    let t17360 = t1882 * t4819;
    let t17362 = t1882 * t4815;
    let t17365 = t925 * t3478;
    let t17366 = t9144 * t17365;
    let t17369 = t925 * t3483;
    let t17370 = t13220 * t17369;
    let t17373 = 2.0 / 9.0 * t1901 * t17195 + t1901 * t17200 / 9.0 - 10.0 / 81.0 * t1901 * t17204 - 8.0 / 27.0 * t11593 * t17208 - t13040 - t13042 - t13049 + t13062 - t446 * t17357 / 3.0 + t17360 / 9.0 + 2.0 / 9.0 * t17362 + 4.0 / 27.0 * t13075 + t13084 - 2.0 / 9.0 * t1901 * t17366 - 4.0 / 9.0 * t1901 * t17370;
    (t17356, t17373)
}
