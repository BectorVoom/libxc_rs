//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1295/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1295<F: Float>(t11228: F, t25202: F, t13675: F, t190: F, t25813: F, t35455: F, t21369: F, t2936: F, t11234: F, t11235: F, t15284: F, t4296: F, t674: F) -> (F, F, F, F, F) {
    let t35506 = t11228 * t25202;
    let t35510 = t35455 * t13675 * t190 * t25813;
    let t35512 = t2936 * t21369;
    let t35515 = t11234 * t11235 * t15284;
    let t35517 = t4296 * t674;
    (t35506, t35510, t35512, t35515, t35517)
}
