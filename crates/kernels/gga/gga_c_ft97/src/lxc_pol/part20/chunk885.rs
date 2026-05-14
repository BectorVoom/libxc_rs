//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 885/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk885<F: Float>(t2568: F, t27986: F, t10002: F, t6930: F, t263: F, t6837: F, t684: F, t2354: F, t10157: F, t3837: F, t6003: F, t1091: F, t24240: F, t24245: F, t10052: F, t1403: F, t24204: F, t24238: F, t24253: F, t27965: F, t27969: F, t27971: F, t27976: F, t27981: F, t27984: F, t6002: F, t6749: F) -> (F, F, F, F, F, F, F, F) {
    let t27987 = t2568 * t27986;
    let t27989 = t10002 * t6930;
    let t27991 = t6837 * t263;
    let t27992 = t27991 * t684;
    let t27993 = t2354 * t27992;
    let t27997 = t10157 * t6003 * t3837;
    let t28001 = t24240 * t1091;
    let t28002 = t2354 * t28001;
    let t28006 = t2354 * t24245 * t1091;
    let t28009 = -t1403 * t27965 / 3.0 + t27969 / 9.0 - 12.0 * t10052 * t27971 - t1403 * t27976 / 3.0 + t24238 / 54.0 - t24253 / 18.0 - 2.0 * t27981 + 4.0 * t27984 + 4.0 * t27987 + 4.0 * t27989 - t6002 * t27993 / 18.0 + t6002 * t27997 - t24204 * t6749 / 18.0 - t6002 * t28002 / 18.0 - t6002 * t28006 / 18.0;
    (t27987, t27989, t27991, t27993, t27997, t28002, t28006, t28009)
}
