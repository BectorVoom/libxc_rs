//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 306/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk306(t1811: f64, t183: f64, t155: f64, t1436: f64, t1027: f64, t1044: f64, t1133: f64, t1813: f64, t1814: f64, t1815: f64, t1816: f64, t1817: f64, t975: f64) -> (f64, f64, f64, f64) {
    let t1842 = t1811 * t183;
    let t1843 = t155 * t1842;
    let t1844 = 0.36622894612013090108e-3_f64 * t1436;
    let t1845 = t1843 + t1813 + t1815 - t1814 - t1816 - t1817 - t1844 - t1044 - t975 + t1133 - t1027;
    (t1842, t1843, t1844, t1845)
}
