//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 670/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk670<F: Float>(t363: F, t979: F, t2983: F, t11556: F, t11552: F, t15955: F, t3214: F, t925: F, t8557: F, t3219: F, t11854: F, t2992: F, t11472: F, t11882: F, t11883: F, t16288: F, t16293: F, t16296: F, t16298: F, t16300: F, t16302: F, t16306: F, t16309: F, t1901: F, t446: F) -> (F, F, F, F, F) {
    let t16312 = t979 * t363;
    let t16313 = t2983 * t16312;
    let t16314 = t11556 * t16313;
    let t16317 = t11552 * t15955;
    let t16320 = t925 * t3214;
    let t16321 = t8557 * t16320;
    let t16324 = t925 * t3219;
    let t16325 = t11854 * t16324;
    let t16328 = t2992 * t16312;
    let t16329 = t11472 * t16328;
    let t16332 = t446 * t16288 / 3.0 + 2.0 / 3.0 * t446 * t16293 - 2.0 / 9.0 * t16296 + 2.0 / 81.0 * t16298 + t16300 / 27.0 + 2.0 / 27.0 * t16302 + t11882 - 8.0 / 81.0 * t11883 - 2.0 / 9.0 * t1901 * t16306 - 4.0 / 9.0 * t1901 * t16309 + 4.0 / 27.0 * t1901 * t16314 + 4.0 / 27.0 * t1901 * t16317 - 2.0 / 9.0 * t1901 * t16321 - 4.0 / 9.0 * t1901 * t16325 - 4.0 / 9.0 * t1901 * t16329;
    (t16313, t16320, t16324, t16328, t16332)
}
