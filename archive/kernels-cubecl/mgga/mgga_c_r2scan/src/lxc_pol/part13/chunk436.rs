//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 436/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk436<F: Float>(t1859: F, t423: F, t170: F, t591: F, t597: F, t584: F, t608: F, t1399: F, t1714: F, t1717: F) -> (F, F, F, F, F, F, F) {
    let t1860 = t1859 * t423;
    let t1861 = t170 * t591;
    let t1862 = t597 * t1861;
    let t1863 = t1860 * t1862;
    let t1866 = t584 * t608 * t591;
    let t1870 = F::cast_from(0.13949e-1_f64) * t1399;
    let t1871 = -F::cast_from(0.24694444444444444445e-2_f64) * t1714 + F::cast_from(0.19755555555555555556e-1_f64) * t1717 + t1870;
    (t1860, t1861, t1862, t1863, t1866, t1870, t1871)
}
