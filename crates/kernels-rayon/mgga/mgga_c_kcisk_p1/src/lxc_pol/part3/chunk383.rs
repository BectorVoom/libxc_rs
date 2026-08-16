//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 383/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk383(t1944: f64, t740: f64, t748: f64, t1872: f64, t1894: f64, t747: f64, t746: f64, t1757: f64, t641: f64, t741: f64, t1932: f64, t1938: f64, t1942: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1945 = t1944 * t740;
    let t1946 = t1945 * t748;
    let t1948 = t1872 * t740;
    let t1949 = t747 * t1894;
    let t1950 = t746 * t1949;
    let t1951 = t1948 * t1950;
    let t1953 = t641 * t1757;
    let t1954 = t746 * t1953;
    let t1955 = t741 * t1954;
    let t1957 = t1932 / 16.0_f64 - t1938 / 16.0_f64 + t1942 / 24.0_f64 - t1946 / 256.0_f64 + t1951 / 256.0_f64 - t1955 / 192.0_f64;
    (t1945, t1946, t1948, t1950, t1951, t1954, t1955, t1957)
}
