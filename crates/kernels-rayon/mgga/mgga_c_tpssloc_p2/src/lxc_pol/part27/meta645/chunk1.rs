//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2208/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2208(t23512: f64, t25486: f64, t23519: f64, t25492: f64, t1607: f64, t23515: f64, t23521: f64, t23529: f64, t4636: f64, t6747: f64, t82911: f64, t82951: f64, t82953: f64, t83092: f64, t88335: f64, t88336: f64, t88339: f64, t88341: f64, t88342: f64, t88348: f64) -> f64 {
    let t88351 = t23512 * t25486;
    let t88354 = t23519 * t25492;
    let t88358 = -t23529 * t4636 / 216.0_f64 + 11.0_f64 / 324.0_f64 * t83092 * t1607 - t88335 - t88336 / 1296.0_f64 + t88339 - t88341 + 0.20186378047070195428e-3_f64 * t88342 * t23515 - 0.10093189023535097714e-3_f64 * t88342 * t23521 - 0.10093189023535097714e-3_f64 * t82951 + 0.16149102437656156342e-2_f64 * t88348 * t6747 - 0.40372756094140390856e-3_f64 * t82911 * t88351 + 0.20186378047070195428e-3_f64 * t82911 * t88354 + t82953 / 1152.0_f64;
    t88358
}
