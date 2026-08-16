//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 526/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk526(t1614: f64, t51: f64, t213: f64, t1109: f64, t679: f64, t689: f64, t1127: f64, t709: f64) -> (f64, f64, f64, f64, f64) {
    let t17839 = t51 * t1614;
    let t17840 = t17839 * t213;
    let t17841 = t1109 * t679;
    let t17842 = t17841 * t689;
    let t17843 = t17840 * t17842;
    let t17859 = t1127 * t709;
    let t17863 = t1127 * t679;
    let t17864 = t17863 * t689;
    (t17839, t17842, t17843, t17859, t17864)
}
