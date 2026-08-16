//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 471/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk471(t1917: f64, t703: f64, t1830: f64, t1880: f64, t1833: f64, t1845: f64, t1863: f64, t1868: f64, t1874: f64, t1876: f64, t1883: f64, t1887: f64, t1891: f64) -> (f64, f64, f64, f64) {
    let t1918 = t1917 * t703;
    let t1923 = 0.68863333333333333333e0_f64 * t1830;
    let t1928 = 0.17365833333333333333e0_f64 * t1880;
    let t1932 = -0.17648625e1_f64 * t1863 + 0.3529725e1_f64 * t1868 + t1923 - 0.103295e1_f64 * t1833 + 0.1549425e1_f64 * t1845 + 0.31558125e0_f64 * t1874 + 0.6311625e0_f64 * t1876 + t1928 - 0.41678e0_f64 * t1883 + 0.312585e0_f64 * t1887 + 0.312585e0_f64 * t1891;
    (t1918, t1923, t1928, t1932)
}
