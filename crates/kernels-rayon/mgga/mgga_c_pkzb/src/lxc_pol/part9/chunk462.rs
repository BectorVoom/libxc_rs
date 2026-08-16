//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 462/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk462(t1885: f64, t218: f64, t219: f64, t1843: f64, t208: f64, t1833: f64, t1845: f64, t1863: f64, t1868: f64, t1870: f64, t1874: f64, t1876: f64, t1881: f64, t1883: f64) -> (f64, f64, f64, f64) {
    let t1887 = t218 * t219 * t1885;
    let t1889 = t208 * t1843;
    let t1891 = t218 * t219 * t1889;
    let t1893 = -0.9494625e0_f64 * t1863 + 0.1898925e1_f64 * t1868 + t1870 - 0.59793333333333333334e0_f64 * t1833 + 0.8969e0_f64 * t1845 + 0.15358125e0_f64 * t1874 + 0.3071625e0_f64 * t1876 + t1881 - 0.32862666666666666666e0_f64 * t1883 + 0.24647e0_f64 * t1887 + 0.24647e0_f64 * t1891;
    (t1887, t1889, t1891, t1893)
}
