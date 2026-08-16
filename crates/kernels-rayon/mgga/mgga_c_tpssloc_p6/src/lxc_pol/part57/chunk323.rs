//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 323/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk323(t1949: f64, t345: f64, t1945: f64, t383: f64, t1920: f64, t353: f64, t1055: f64, t1052: f64, t1923: f64, t1946: f64, t388: f64, t1914: f64, t202: f64) -> (f64, f64, f64, f64, f64) {
    let t1950 = t345 * t1949;
    let t1953 = t383 * t1945;
    let t1955 = 0.82246703342411321825e-2_f64 * t1920 * t1950 + t353 * t1953;
    let t1956 = t1055 * t1955;
    let t1958 = 0.82246703342411321825e-2_f64 * t1920 * t1923 + t1946 * t388 - t1052 * t1956;
    let t1962 = t202 * t1914;
    (t1953, t1955, t1956, t1958, t1962)
}
