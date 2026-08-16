//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1091/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1091(t30663: f64, t7479: f64, t6552: f64, t7488: f64, t1880: f64, t1527: f64, t8352: f64, t10110: f64, t1911: f64, t7537: f64, t2718: f64, t8362: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32789 = t30663 * t7479;
    let t32791 = 0.3289868133696452873e-1_f64 * t6552 * t32789;
    let t32792 = t30663 * t7488;
    let t32794 = 0.16449340668482264365e-1_f64 * t1880 * t32792;
    let t32795 = t8352 * t1527;
    let t32796 = t10110 * t32795;
    let t32799 = t1911 * t7537;
    let t32800 = t2718 * t32799;
    let t32803 = t8362 * t1527;
    (t32789, t32791, t32792, t32794, t32796, t32800, t32803)
}
