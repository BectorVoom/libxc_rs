//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1214/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1214(t14338: f64, t14381: f64, t14435: f64, t15081: f64, t2: f64, t895: f64, t580: f64, t265: f64, t22: f64, t4567: f64, t1610: f64, t2875: f64) -> (f64, f64, f64, f64, f64) {
    let t15083 = t14338 + t14381 + t14435 + t15081;
    let t15091 = t895 * t2;
    let t15093 = 2.0_f64 * t15091 * t580;
    let t15094 = t265 * t580;
    let t15096 = 3.0_f64 * t4567 * t22;
    let t15098 = t1610 * t2875;
    (t15083, t15093, t15094, t15096, t15098)
}
