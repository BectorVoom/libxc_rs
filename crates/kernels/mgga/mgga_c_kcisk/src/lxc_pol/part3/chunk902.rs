//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 902/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk902<F: Float>(t529: F, t13900: F, t1582: F, t1580: F, t3973: F, t4407: F, t13820: F, t1579: F, t4381: F, t4384: F, t13125: F, t41: F, t13394: F, t6443: F, t1287: F, t13778: F, t13785: F, t1558: F, t382: F, t4144: F, t4148: F, t4354: F, t525: F, t526: F, t6442: F) -> (F, F, F, F, F) {
    let t530 = t529 < -0.66725e-1;
    let t15005 = t13900 * t1582;
    let t15006 = t1580 * t15005;
    let t15008 = t3973 * t4407;
    let t15009 = t1580 * t15008;
    let t15011 = t1579 * t13820;
    let t15014 = t4381 * t4384;
    let t15016 = t13125 * t41;
    let t15032 = t6443 * t13394;
    let t15039 = piecewise3(t530, 0.0, 10.0 / 9.0 * t525 * t15016 * t382 - 10.0 / 9.0 * t525 * t4354 * t1287 + 40.0 / 27.0 * t525 * t1558 * t4144 - 10.0 / 9.0 * t525 * t1558 * t4148 - 280.0 / 243.0 * t525 * t526 * t13778 + 40.0 / 27.0 * t6442 * t15032 - 10.0 / 27.0 * t525 * t526 * t13785);
    (t15006, t15009, t15011, t15014, t15039)
}
