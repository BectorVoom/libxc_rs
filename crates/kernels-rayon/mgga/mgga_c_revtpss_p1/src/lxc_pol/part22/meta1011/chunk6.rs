//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3475/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3475(t2986: f64, t63902: f64, t973: f64, t981: f64, t19468: f64, t3022: f64, t19021: f64, t974: f64, t2988: f64, t41235: f64, t41238: f64, t6189: f64) -> (f64, f64, f64, f64) {
    let t65402 = 0.23392894490538584828e1_f64 * t981 * t2986 * t63902 * t973;
    let t65404 = 0.34631718211362927518e2_f64 * t3022 * t19468;
    let t65408 = 0.23392894490538584828e1_f64 * t981 * t2986 * t19021 * t974;
    let t65413 = 0.91082604192152556044e5_f64 * t981 * t41235 * t6189 * t41238 * t2988;
    (t65402, t65404, t65408, t65413)
}
