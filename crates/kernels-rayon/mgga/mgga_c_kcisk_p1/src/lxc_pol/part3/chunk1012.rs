//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1012/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1012(t3973: f64, t4387: f64, t1580: f64, t13917: f64, t4392: f64, t12924: f64, t1581: f64, t1312: f64, t12829: f64, t539: f64, t12830: f64, t13894: f64) -> (f64, f64, f64, f64) {
    let t14921 = t3973 * t4387;
    let t14922 = t1580 * t14921;
    let t14924 = t13917 * t4392;
    let t14925 = t1580 * t14924;
    let t14929 = t1581 * t12924;
    let t14930 = t1312 * t14929;
    let t14935 = t539 * t12829;
    let t14936 = t14935 * t12830;
    let t14937 = t13894 * t14936;
    (t14922, t14925, t14930, t14937)
}
