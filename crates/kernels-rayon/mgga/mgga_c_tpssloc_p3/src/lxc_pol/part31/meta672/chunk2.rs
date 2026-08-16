//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2015/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2015(t1824: f64, t7918: f64, t1332: f64, t1352: f64, t19735: f64, t19805: f64, t2089: f64, t27074: f64, t29327: f64, t5250: f64, t5287: f64, t5334: f64, t5344: f64, t90868: f64, t90876: f64, t93524: f64, t93528: f64, t93529: f64, t93537: f64, t96962: f64, t96967: f64, t96972: f64, t96976: f64, t96979: f64) -> f64 {
    let t102562 = t7918 * t1824;
    let t102580 = -0.19739208802178717238e0_f64 * t96962 + t19805 * t2089 + t1332 * t29327 - t93524 + 4.0_f64 * t5334 * t102562 * t5250 - 2.0_f64 * t5344 * t27074 * t5287 + t93528 + t93529 + 4.0_f64 * t5334 * t27074 * t19735 - t93537 + 0.25587863262083522345e0_f64 * t90868 + 0.6579736267392905746e-1_f64 * t96967 - 2.0_f64 * t5344 * t102562 * t1352 + 0.3289868133696452873e-1_f64 * t96972 + 0.3289868133696452873e-1_f64 * t96976 - 0.3289868133696452873e-1_f64 * t96979 + t90876;
    t102580
}
