//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 317/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk317(t1767: f64, t326: f64, t128: f64, t1743: f64, t1763: f64, t874: f64, t118: f64, t1756: f64, t338: f64, t1927: f64, t1929: f64, t1931: f64, t1934: f64, t1937: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1939 = t326 * t1767;
    let t1941 = t128 * t1743;
    let t1942 = t326 * t1941;
    let t1944 = t874 * t1763;
    let t1945 = t118 * t1944;
    let t1947 = t338 * t1756;
    let t1948 = t118 * t1947;
    let t1950 = -0.11974241701863808564e0_f64 * t1927 + 0.35922725105591425692e0_f64 * t1929 + 0.11974241701863808564e0_f64 * t1931 - 0.59871208509319042821e-1_f64 * t1934 - 0.23948483403727617128e0_f64 * t1937 - 0.11974241701863808564e0_f64 * t1939 + 0.59871208509319042821e-1_f64 * t1942 - 0.39914139006212695214e-1_f64 * t1945 + 0.19957069503106347607e-1_f64 * t1948;
    (t1939, t1941, t1942, t1944, t1945, t1947, t1948, t1950)
}
