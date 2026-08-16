//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1016/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1016(t13900: f64, t1582: f64, t1580: f64, t3973: f64, t4407: f64, t13820: f64, t1579: f64, t4381: f64, t4384: f64, t13125: f64, t41: f64, t13394: f64, t6443: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15005 = t13900 * t1582;
    let t15006 = t1580 * t15005;
    let t15008 = t3973 * t4407;
    let t15009 = t1580 * t15008;
    let t15011 = t1579 * t13820;
    let t15014 = t4381 * t4384;
    let t15016 = t13125 * t41;
    let t15032 = t6443 * t13394;
    (t15006, t15009, t15011, t15014, t15016, t15032)
}
