//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 913/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk913(t1102: f64, t672: f64, t1098: f64, t1127: f64, t650: f64, t1015: f64, t242: f64, t1125: f64, t2845: f64, t400: f64, t2192: f64, t359: f64, t461: f64) -> (f64, f64, f64, f64, f64) {
    let t9657 = t672 * t1102;
    let t9658 = t1098 * t9657;
    let t9666 = t650 * t1127;
    let t9668 = t242 * t9666 * t1015;
    let t9669 = t1125 * t9668;
    let t9684 = 1.0_f64 / t400 / t2845;
    let t9699 = t359 * t2192 * t461;
    (t9658, t9666, t9669, t9684, t9699)
}
