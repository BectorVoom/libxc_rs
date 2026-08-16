//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 854/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk854(t214: f64, t32761: f64, t1985: f64, t26193: f64, t8458: f64, t30663: f64, t7479: f64, t6552: f64, t7488: f64, t1880: f64, t225: f64, t258: f64, t7510: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32762 = t214 * t32761;
    let t32764 = 0.16449340668482264365e-1_f64 * t1985 * t32762;
    let t32769 = t26193 * t8458;
    let t32771 = 0.16449340668482264365e-1_f64 * t1985 * t32769;
    let t32789 = t30663 * t7479;
    let t32791 = 0.3289868133696452873e-1_f64 * t6552 * t32789;
    let t32792 = t30663 * t7488;
    let t32794 = 0.16449340668482264365e-1_f64 * t1880 * t32792;
    let t32808 = t7510 * t225 * t258;
    (t32762, t32764, t32769, t32771, t32789, t32791, t32792, t32794, t32808)
}
