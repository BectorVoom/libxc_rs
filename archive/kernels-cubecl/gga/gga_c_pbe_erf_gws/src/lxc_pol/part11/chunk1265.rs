//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1265/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1265<F: Float>(t46104: F, t38735: F, t3854: F, t2157: F, t343: F, t49847: F, t46115: F, t11419: F, t44479: F, t11414: F, t37755: F, t858: F, t866: F, t867: F) -> (F, F, F, F, F, F, F, F, F) {
    let t50115 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t46104;
    let t50116 = F::cast_from(35.0_f64) / F::cast_from(18.0_f64) * t38735;
    let t50117 = t3854 * t3854;
    let t50118 = t50117 * t2157;
    let t50123 = t49847 * t343;
    let t50128 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t46115;
    let t50135 = t11419 * t44479 / F::cast_from(4.0_f64);
    let t50137 = t37755 * t11414 / F::cast_from(4.0_f64);
    let t50142 = t50117 * t343;
    let t50146 = t866 * t867 * t858 * t50142 / F::cast_from(32.0_f64);
    (t50115, t50116, t50118, t50123, t50128, t50135, t50137, t50142, t50146)
}
