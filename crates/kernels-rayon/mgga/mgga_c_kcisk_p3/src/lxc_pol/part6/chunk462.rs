//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 462/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk462(t3783: f64, t453: f64, t1413: f64, t394: f64, sigma0: f64) -> (f64, f64, f64, f64) {
    let t3784 = t453 * t3783;
    let t3785 = t3784 * sigma0;
    let t3795 = t1413 * sigma0;
    let t3796 = t3795 * t394;
    (t3784, t3785, t3795, t3796)
}
