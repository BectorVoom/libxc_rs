//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1419/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1419(t11277: f64, t3307: f64, t11275: f64, t3265: f64, t11400: f64, t11628: f64, t1164: f64, t11285: f64, t3395: f64, t11282: f64, t3377: f64, t11403: f64, t11424: f64) -> (f64, f64, f64, f64, f64) {
    let t43976 = t3307 * t11277;
    let t43979 = 0.3103560775156404018e4_f64 * t11275 * t43976 * t3265;
    let t43982 = 0.46785788981077169656e1_f64 * t1164 * t11628 * t11400;
    let t43984 = t11285 * t3395;
    let t43987 = 0.61524113149298439947e4_f64 * t1164 * t11282 * t3377 * t43984;
    let t43989 = 24.0_f64 * t11424 * t11403;
    (t43979, t43982, t43984, t43987, t43989)
}
