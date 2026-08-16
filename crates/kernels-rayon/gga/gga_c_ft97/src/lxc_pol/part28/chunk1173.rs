//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1173/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1173(t1882: f64, t35118: f64, t12664: f64, t33090: f64, t23997: f64, t26526: f64, t1017: f64, t12703: f64, t140378: f64, t140382: f64, t140383: f64, t144: f64, t148403: f64, t148408: f64, t148412: f64, t1901: f64, t2142: f64, t2179: f64, t2185: f64, t27256: f64, t32962: f64, t33060: f64, t33125: f64, t33227: f64, t3408: f64, t3429: f64, t35050: f64, t35192: f64, t3590: f64, t40792: f64, t40945: f64, t446: f64, t47659: f64, t51151: f64, t574: f64, t5975: f64, t605: f64, t6615: f64, t7312: f64, t7400: f64, t7407: f64, t9144: f64, t925: f64, t95842: f64) -> (f64, f64, f64) {
    let t149132 = t1882 * t35118;
    let t149141 = t12664 * t33090;
    let t149191 = t23997 * t26526;
    let t149196 = 2.0_f64 / 3.0_f64 * t140378 + 2.0_f64 / 9.0_f64 * t149132 - 2.0_f64 / 3.0_f64 * t446 * t574 * t5975 * t6615 + t140382 + 4.0_f64 / 9.0_f64 * t47659 * t95842 * t27256 + 4.0_f64 / 3.0_f64 * t446 * t144 * t149141 + t446 * t574 * t605 * t7407 * t3408 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t574 * t2179 * t7400 * t3408 + 2.0_f64 / 3.0_f64 * t446 * t2185 * t3590 * t7312 - 2.0_f64 / 9.0_f64 * t1901 * t40945 * t35192 - 2.0_f64 / 9.0_f64 * t1901 * t9144 * t33060 * t925 - 2.0_f64 / 9.0_f64 * t1901 * t9144 * t33125 * t925 + 2.0_f64 / 3.0_f64 * t1901 * t51151 * t148403 + 2.0_f64 / 9.0_f64 * t1901 * t40792 * t32962 * t3429 - 4.0_f64 / 9.0_f64 * t1901 * t12703 * t148408 + 4.0_f64 / 9.0_f64 * t1901 * t12703 * t148412 - t446 * t574 * t33227 * t1017 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t2185 * t2142 * t35050 + 4.0_f64 / 3.0_f64 * t446 * t144 * t149191 - 2.0_f64 / 9.0_f64 * t140383;
    (t149141, t149191, t149196)
}
