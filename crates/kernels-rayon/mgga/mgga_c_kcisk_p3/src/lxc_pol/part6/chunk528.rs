//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 528/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk528(t2063: f64, t4597: f64, t1849: f64, t1646: f64, t2484: f64, t2372: f64, t4663: f64, t1644: f64, t2368: f64, t4716: f64, t2378: f64, t827: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6758 = t4597 * t2063;
    let t6763 = t1849 * t2063;
    let t6774 = t1646 * t2484;
    let t6777 = t4663 * t2372;
    let t6802 = t2368 * t1644;
    let t6817 = t4716 * t2372;
    let t6823 = t827 * t2378;
    (t6758, t6763, t6774, t6777, t6802, t6817, t6823)
}
