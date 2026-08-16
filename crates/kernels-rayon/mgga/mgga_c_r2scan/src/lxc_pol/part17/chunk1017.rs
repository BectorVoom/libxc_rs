//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1017/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1017(t10610: f64, t12739: f64, t12383: f64, t3472: f64, t3275: f64, t1149: f64, t2995: f64, t12056: f64, t3262: f64, t3574: f64, t3465: f64, t8601: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12740 = t10610 * t12739;
    let t12741 = 3.0_f64 / 2.0_f64 * t12740;
    let t12742 = t3472 * t12383;
    let t12743 = t3275 * t12742;
    let t12744 = 5.0_f64 / 8.0_f64 * t12743;
    let t12745 = t2995 * t1149;
    let t12747 = t3262 * t12056 * t3574;
    let t12748 = 3.0_f64 / 2.0_f64 * t12747;
    let t12751 = t3275 * t3465 * t8601;
    (t12741, t12742, t12744, t12745, t12748, t12751)
}
