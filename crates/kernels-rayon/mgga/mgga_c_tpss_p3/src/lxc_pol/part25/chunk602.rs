//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 602/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk602(t1163: f64, t1338: f64, t1334: f64, t600: f64, t1333: f64, t2073: f64, t640: f64, t1324: f64, t2083: f64, t633: f64, t100: f64, t2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3502 = t1163 * t1338;
    let t3506 = t600 * t1334;
    let t3508 = t2073 * t1333;
    let t3509 = t3508 * t640;
    let t3514 = t2083 * t1324;
    let t3515 = t3514 * t633;
    let t3518 = t100 * t2;
    (t3502, t3506, t3508, t3509, t3515, t3518)
}
