//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 319/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk319(t1359: f64, t265: f64, t1327: f64, t1334: f64, t1337: f64, t1340: f64, t831: f64, t834: f64) -> (f64, f64) {
    let t1360 = t1359 * t265;
    let t1366 = 0.258925e1_f64 * t1334 - t831 - 0.301925e0_f64 * t1327 + 0.16504875e0_f64 * t1337 - t834 - 0.82785e-1_f64 * t1340;
    (t1360, t1366)
}
