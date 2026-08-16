//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 923/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk923(t1248: f64, t3583: f64, t3979: f64, t1237: f64, t4037: f64, t4046: f64, t4054: f64, t12983: f64, t4065: f64, t1249: f64, t12868: f64, t3118: f64, t313: f64, t353: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13650 = t1248 * t3979 * t3583;
    let t13653 = t4037 * t1237 * t4046;
    let t13656 = t4054 * t1237 * t4046;
    let t13659 = t1248 * t4065 * t12983;
    let t13662 = t1248 * t1249 * t12868;
    let t13665 = t353 * t3118 * t313;
    (t13650, t13653, t13656, t13659, t13662, t13665)
}
