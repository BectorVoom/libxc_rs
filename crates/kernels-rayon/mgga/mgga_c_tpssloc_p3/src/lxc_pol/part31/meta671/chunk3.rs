//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2004/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2004(t55921: f64, t7025: f64, t2240: f64, t5392: f64, t63: f64, t2032: f64, t26067: f64, t26911: f64, t27966: f64, t28935: f64, t6492: f64, t6495: f64, t7026: f64, t7035: f64, t91959: f64, t96393: f64, t96406: f64, t96479: f64, t96482: f64, t96517: f64, t96521: f64) -> f64 {
    let t102267 = t55921 * t7025;
    let t102275 = t2240 * t5392 * t63;
    let t102278 = -10.0_f64 / 3.0_f64 * t26911 * t26067 - 4.0_f64 / 3.0_f64 * t96406 * t2032 - 4.0_f64 / 3.0_f64 * t96479 * t2032 - 4.0_f64 / 3.0_f64 * t96482 * t2032 - 4.0_f64 / 3.0_f64 * t27966 * t7035 - 5.0_f64 / 3.0_f64 * t7026 * t96393 - 2.0_f64 / 3.0_f64 * t6495 * t28935 - 5.0_f64 / 3.0_f64 * t102267 * t6492 - 5.0_f64 / 3.0_f64 * t7026 * t96517 - 5.0_f64 / 3.0_f64 * t7026 * t96521 + 10.0_f64 / 3.0_f64 * t102275 * t6492 + t91959;
    t102278
}
