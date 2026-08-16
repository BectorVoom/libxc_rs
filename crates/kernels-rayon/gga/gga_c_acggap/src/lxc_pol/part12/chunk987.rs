//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 987/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk987(t7990: f64, t8081: f64, t2131: f64, t2147: f64, t463: f64, t8099: f64, t323: f64, t3242: f64, t633: f64, t32092: f64, t8313: f64, t30029: f64, t8310: f64) -> (f64, f64, f64, f64, f64) {
    let t33210 = t7990 * t8081;
    let t33214 = t2131 * t2147 * t8099 * t463;
    let t33227 = 0.19756347548806534796e1_f64 * t3242 * t633 * t323;
    let t33228 = t32092 * t8313;
    let t33230 = t30029 * t8310;
    (t33210, t33214, t33227, t33228, t33230)
}
