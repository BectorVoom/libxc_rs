//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1286/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1286(t114: f64, t2340: f64, t94978: f64, t2366: f64, t25823: f64, t10208: f64, t68: f64, t10209: f64, t665: f64, t25826: f64, t10254: f64, t6998: f64, t94974: f64, t94976: f64) -> f64 {
    let t115 = 1.0_f64 < t114;
    let t94979 = t94978 * t2340;
    let t94981 = t25823 * t2366;
    let t94982 = t68 * t10208;
    let t94983 = t94982 * t10209;
    let t94985 = t665 * t2366;
    let t94986 = t25826 * t94985;
    let t94988 = t6998 * t10254;
    let t94991 = piecewise3(t115, 0.0_f64, -t94974 - 11.0_f64 / 3.0_f64 * t94976 - 2.0_f64 * t94979 + t94981 - 3.0_f64 / 4.0_f64 * t94983 + 3.0_f64 / 4.0_f64 * t94986 - t94988 / 8.0_f64);
    t94991
}
