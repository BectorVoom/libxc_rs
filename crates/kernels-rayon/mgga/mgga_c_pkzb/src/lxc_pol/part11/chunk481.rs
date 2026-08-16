//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 481/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk481(t154: f64, t2347: f64, t824: f64, t385: f64, t52: f64, t931: f64, t395: f64, t912: f64, t2016: f64) -> (f64, f64, f64, f64, f64) {
    let t2349 = t154 * t2347 * t824;
    let t2350 = t385 * t2349;
    let t2352 = t52 * t931;
    let t2362 = 1.0_f64 / t912 / t395;
    let t2363 = t2016 * t2362;
    (t2349, t2350, t2352, t2362, t2363)
}
