//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2028/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2028(t12020: f64, t7213: f64, t90723: f64, t12444: f64, t1375: f64, t1385: f64, t16453: f64, t1807: f64, t2092: f64, t24063: f64, t26990: f64, t27114: f64, t3887: f64, t55093: f64, t568: f64, t7194: f64, t7937: f64, t81307: f64, t81311: f64, t90665: f64, t90728: f64, t90737: f64, t90741: f64) -> (f64, f64, f64) {
    let t93818 = t12020 * t7213;
    let t93824 = 0.16449340668482264365e-1_f64 * t90723;
    let t93847 = -12.0_f64 * t90665 * t26990 - 2.0_f64 * t55093 * t2092 + 0.3289868133696452873e-1_f64 * t90728 + 4.0_f64 * t7194 * t16453 + 4.0_f64 * t1375 * t3887 * t27114 * t1385 - 2.0_f64 * t12444 * t7937 - 0.38381794893125283518e-1_f64 * t81307 - 0.16449340668482264365e-1_f64 * t90737 - 0.6579736267392905746e-1_f64 * t90741 + t1807 * t24063 * t568 - 0.3289868133696452873e-1_f64 * t81311;
    (t93818, t93824, t93847)
}
