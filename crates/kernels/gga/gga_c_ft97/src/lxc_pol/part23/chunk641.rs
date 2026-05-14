//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 641/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk641<F: Float>(t1775: F, t4207: F, t4200: F, t10580: F, t2: F, t4215: F, t1232: F, t1771: F, t4224: F, t458: F, t11717: F, t4210: F, t1228: F, t8282: F, t4220: F, t2347: F, t852: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14955 = 4.0 / 9.0 * t1775 * t4207;
    let t14957 = 4.0 / 27.0 * t1775 * t4200;
    let t14961 = t10580 * t2;
    let t14999 = 2.0 / 9.0 * t1775 * t4215;
    let t15011 = t1771 * t1232;
    let t15014 = 2.0 / 3.0 * t458 * t4224;
    let t15015 = t11717 * t4210;
    let t15025 = t8282 * t1228;
    let t15028 = 4.0 / 3.0 * t1775 * t4220;
    let t15042 = t852 * t2347;
    (t14955, t14957, t14961, t14999, t15011, t15014, t15015, t15025, t15028, t15042)
}
