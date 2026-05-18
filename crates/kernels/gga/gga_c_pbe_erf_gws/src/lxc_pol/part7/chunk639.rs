//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 639/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk639<F: Float>(t219: F, t5002: F, t4367: F, t1640: F, t639: F, t197: F, t4957: F, t4352: F, t1661: F, t587: F, t1866: F, t562: F, t597: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5003 = t219 * t5002;
    let t5004 = t5003 * t4367;
    let t5005 = t1640 * t5004;
    let t5007 = F::new(8.0) / F::new(9.0) * t639 * t5005;
    let t5008 = t197 * t4957;
    let t5009 = t5008 * t4352;
    let t5010 = t1661 * t5009;
    let t5012 = F::new(8.0) / F::new(9.0) * t587 * t5010;
    let t5014 = t597 * t1866 * t562;
    (t5003, t5004, t5005, t5007, t5008, t5009, t5010, t5012, t5014)
}
