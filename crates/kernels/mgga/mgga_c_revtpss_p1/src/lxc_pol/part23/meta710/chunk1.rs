//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2467/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2467<F: Float>(t14293: F, t9664: F, t14103: F, t9285: F, t9674: F, t13726: F, t9303: F, t13725: F, t1445: F, t2439: F, t14082: F, t3920: F) -> (F, F, F, F, F) {
    let t47920 = t14293 * t9664;
    let t47932 = t9674 * t14103 * t9285;
    let t47938 = t9303 * t13726;
    let t47942 = t2439 * t13725 * t1445;
    let t47944 = t14082 * t3920;
    (t47920, t47932, t47938, t47942, t47944)
}
