//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2129/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2129<F: Float>(t15993: F, t4574: F, t1011: F, t15145: F, t4915: F, t15149: F, t15154: F, t4919: F, t15130: F, t15135: F, t1012: F, t11821: F) -> (F, F, F, F, F, F, F, F) {
    let t15994 = t15993 * t4574;
    let t15996 = t1011 * t15994 / F::cast_from(324.0_f64);
    let t15997 = t4915 * t15145;
    let t16000 = t4915 * t15149;
    let t16003 = t4919 * t15154;
    let t16006 = t4919 * t15130;
    let t16009 = t4919 * t15135;
    let t16012 = t1012 * t11821;
    (t15994, t15996, t15997, t16000, t16003, t16006, t16009, t16012)
}
