//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 422/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk422(t1936: f64, t1976: f64, t2013: f64, t2033: f64, t61: f64, t41: f64, t607: f64, t759: f64, t761: f64, t1393: f64, t1396: f64, t1870: f64) -> (f64, f64, f64, f64, f64) {
    let t2035 = t1936 + t1976 + t2013 + t2033;
    let t2036 = t61 * t2035;
    let t2037 = t41 * t2036;
    let t2045 = t759 * t607 * t761;
    let t2049 = -0.49388888888888888889e-2_f64 * t1393 + 0.98777777777777777777e-2_f64 * t1396 + t1870;
    (t2035, t2036, t2037, t2045, t2049)
}
