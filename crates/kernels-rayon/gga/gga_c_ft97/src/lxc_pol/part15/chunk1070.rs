//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1070/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1070(t39749: f64, t446: f64, t86661: f64, t85483: f64, t9327: f64, t39725: f64, t86637: f64, t27: f64, t40266: f64, t86676: f64, t89: f64, t49337: f64, t78362: f64, t78364: f64, t78366: f64, t78368: f64, t78396: f64, t87056: f64, t87060: f64, t87063: f64, t87067: f64, t87071: f64) -> (f64, f64, f64, f64, f64) {
    let t87074 = t446 * t39749 * t86661;
    let t87077 = t446 * t9327 * t85483;
    let t87080 = t446 * t39725 * t86637;
    let t87084 = t89 * t27 * t40266 * t86676;
    let t87086 = 8.0_f64 / 9.0_f64 * t78362 - 4.0_f64 / 9.0_f64 * t78364 - 4.0_f64 / 9.0_f64 * t78366 + 8.0_f64 / 27.0_f64 * t78368 + 3.0_f64 / 4.0_f64 * t87056 + 112.0_f64 / 243.0_f64 * t49337 + 2.0_f64 / 3.0_f64 * t87060 + 4.0_f64 / 9.0_f64 * t87063 + 40.0_f64 / 27.0_f64 * t87067 + 8.0_f64 / 9.0_f64 * t78396 + 8.0_f64 / 3.0_f64 * t87071 + 8.0_f64 / 3.0_f64 * t87074 - 20.0_f64 / 27.0_f64 * t87077 + 40.0_f64 / 81.0_f64 * t87080 + 8.0_f64 * t87084;
    (t87074, t87077, t87080, t87084, t87086)
}
