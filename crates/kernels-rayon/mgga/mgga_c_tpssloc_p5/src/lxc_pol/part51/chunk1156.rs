//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1156/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1156(t31332: f64, t865: f64, t23270: f64, t1888: f64, t2053: f64, t857: f64, t776: f64, t22986: f64, t1911: f64, t7106: f64, t2718: f64, t1912: f64, t24305: f64, t30655: f64, t30659: f64, t30662: f64, t30666: f64, t30669: f64, t31330: f64, t6627: f64, t7107: f64, t855: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31333 = t31332 * t865;
    let t31334 = t23270 * t31333;
    let t31335 = t1888 * t31334;
    let t31337 = t857 * t2053;
    let t31338 = t31337 * t776;
    let t31339 = t23270 * t31338;
    let t31340 = t22986 * t31339;
    let t31342 = t7106 * t1911;
    let t31343 = t2718 * t31342;
    let t31347 = -t24305 * t1912 - 0.82246703342411321825e-2_f64 * t31330 + 0.16449340668482264365e-1_f64 * t31335 - t30655 + 0.16449340668482264365e-1_f64 * t31340 + 2.0_f64 * t855 * t31343 - t30659 - t6627 * t7107 + t30662 - t30666 - t30669;
    (t31333, t31334, t31337, t31338, t31339, t31343, t31347)
}
