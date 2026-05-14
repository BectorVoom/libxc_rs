//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1057/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1057<F: Float>(t2349: F, t3849: F, t1220: F, t8339: F, t154: F, t2347: F, t385: F, t9795: F, t8329: F, t10189: F, t410: F, t2393: F, t3880: F, t937: F, t10365: F, t2464: F) -> (F, F, F, F, F, F, F) {
    let t28374 = t3849 * t2349;
    let t28376 = t1220 * t8339;
    let t28380 = t385 * t154 * t2347 * t9795;
    let t28384 = t1220 * t8329;
    let t28456 = t410 * t10189;
    let t28457 = t2393 * t28456;
    let t28492 = t937 * t3880;
    let t28493 = t2393 * t28492;
    let t28595 = t10365 * t2464;
    (t28374, t28376, t28380, t28384, t28457, t28493, t28595)
}
