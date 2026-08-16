//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 958/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk958(t1333: f64, t3919: f64, t1163: f64, t4196: f64, t3796: f64, t3482: f64, t13959: f64, t3800: f64, t3734: f64, t3739: f64, t3732: f64, t3764: f64) -> (f64, f64, f64, f64, f64) {
    let t14173 = t1333 * t3919;
    let t14175 = t4196 * t1163;
    let t14176 = t3796 * t14175;
    let t14177 = t3482 * t14176;
    let t14179 = t13959 * t3800;
    let t14181 = t3739 * t3734;
    let t14183 = t3764 * t3732;
    (t14173, t14177, t14179, t14181, t14183)
}
