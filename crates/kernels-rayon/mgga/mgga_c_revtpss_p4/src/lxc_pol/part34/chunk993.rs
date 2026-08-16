//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 993/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk993(t4598: f64, t6120: f64, t4614: f64, t11304: f64, t15189: f64, t18919: f64, t18924: f64, t18934: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23501: f64, t23505: f64) -> (f64, f64, f64) {
    let t23521 = t4598 * t6120;
    let t23523 = t4614 * t6120;
    let t23535 = -t11304 - 4.0_f64 / 9.0_f64 * t15189 + 2.0_f64 / 9.0_f64 * t18919 - 2.0_f64 / 3.0_f64 * t18924 + t18934 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t23479 + 4.0_f64 / 3.0_f64 * t23483 - 2.0_f64 / 3.0_f64 * t23501 - 2.0_f64 * t23487 + 2.0_f64 * t23505 - t23490 / 3.0_f64;
    (t23521, t23523, t23535)
}
