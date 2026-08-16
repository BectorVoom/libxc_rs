//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 529/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk529(t2746: f64, t685: f64, t1084: f64, t1850: f64, t683: f64, t1855: f64, t1073: f64, t1861: f64, t667: f64, t1833: f64, t1865: f64, t2730: f64, t2741: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2748 = 1.0_f64 * t2746 * t685;
    let t2750 = 1.0_f64 * t1850 * t1084;
    let t2751 = t1084 * t683;
    let t2753 = 2.0_f64 * t1855 * t2751;
    let t2754 = t1861 * t1073;
    let t2755 = t2754 * t667;
    let t2759 = t1865 - t1833 / 3.0_f64 - t2730 / 3.0_f64 + t2741;
    (t2748, t2750, t2751, t2753, t2754, t2755, t2759)
}
