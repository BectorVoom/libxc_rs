//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 482/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk482(t1830: f64, t1880: f64, t1833: f64, t1845: f64, t1863: f64, t1868: f64, t1874: f64, t1876: f64, t1883: f64, t1887: f64, t1891: f64) -> (f64, f64, f64) {
    let t1962 = 0.40256666666666666667e0_f64 * t1830;
    let t1967 = 0.137975e0_f64 * t1880;
    let t1971 = -0.1294625e1_f64 * t1863 + 0.258925e1_f64 * t1868 + t1962 - 0.60385e0_f64 * t1833 + 0.905775e0_f64 * t1845 + 0.82524375e-1_f64 * t1874 + 0.16504875e0_f64 * t1876 + t1967 - 0.33114e0_f64 * t1883 + 0.248355e0_f64 * t1887 + 0.248355e0_f64 * t1891;
    (t1962, t1967, t1971)
}
