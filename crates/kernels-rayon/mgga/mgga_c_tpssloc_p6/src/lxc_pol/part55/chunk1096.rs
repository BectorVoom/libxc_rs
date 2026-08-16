//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1096/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1096(t32734: f64, t32780: f64, t533: f64, t1390: f64, t1983: f64, t30663: f64, t7479: f64, t6552: f64, t7488: f64, t1880: f64, t1527: f64, t8352: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32781 = t32734 + t32780;
    let t32782 = t533 * t32781;
    let t32783 = t32782 * t1390;
    let t32784 = t1983 * t32783;
    let t32789 = t30663 * t7479;
    let t32791 = 0.3289868133696452873e-1_f64 * t6552 * t32789;
    let t32792 = t30663 * t7488;
    let t32794 = 0.16449340668482264365e-1_f64 * t1880 * t32792;
    let t32795 = t8352 * t1527;
    (t32781, t32782, t32783, t32784, t32789, t32791, t32792, t32794, t32795)
}
