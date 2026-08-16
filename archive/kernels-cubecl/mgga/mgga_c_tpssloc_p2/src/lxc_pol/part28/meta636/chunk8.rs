//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2028/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2028<F: Float>(t12020: F, t7213: F, t90723: F, t12444: F, t1375: F, t1385: F, t16453: F, t1807: F, t2092: F, t24063: F, t26990: F, t27114: F, t3887: F, t55093: F, t568: F, t7194: F, t7937: F, t81307: F, t81311: F, t90665: F, t90728: F, t90737: F, t90741: F) -> (F, F, F) {
    let t93818 = t12020 * t7213;
    let t93824 = F::cast_from(0.16449340668482264365e-1_f64) * t90723;
    let t93847 = -F::cast_from(12.0_f64) * t90665 * t26990 - F::cast_from(2.0_f64) * t55093 * t2092 + F::cast_from(0.3289868133696452873e-1_f64) * t90728 + F::cast_from(4.0_f64) * t7194 * t16453 + F::cast_from(4.0_f64) * t1375 * t3887 * t27114 * t1385 - F::cast_from(2.0_f64) * t12444 * t7937 - F::cast_from(0.38381794893125283518e-1_f64) * t81307 - F::cast_from(0.16449340668482264365e-1_f64) * t90737 - F::cast_from(0.6579736267392905746e-1_f64) * t90741 + t1807 * t24063 * t568 - F::cast_from(0.3289868133696452873e-1_f64) * t81311;
    (t93818, t93824, t93847)
}
