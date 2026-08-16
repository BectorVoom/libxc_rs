//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1081/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1081<F: Float>(t2567: F, t7440: F, t1882: F, t35699: F, t35734: F, t35684: F, t681: F, t89: F, t141997: F, t142219: F, t142224: F, t142234: F, t142240: F, t1443: F, t151347: F, t1901: F, t193: F, t241: F, t258: F, t28141: F, t28204: F, t3281: F, t3746: F, t3898: F, t724: F, t7560: F, t9707: F, t97777: F) -> (F, F, F) {
    let t152164 = t2567 * t7440;
    let t152179 = t1882 * t35699;
    let t152191 = t1882 * t35734;
    let t152203 = t89 * t681 * t35684;
    let t152218 = -F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t142219 + t152191 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t142224 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t97777 * t28204 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t142234 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3281 * t724 * t7560 * t3746 - t152203 / F::cast_from(9.0_f64) + t142240 / F::cast_from(9.0_f64) + t1901 * t141997 * t3898 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) * t1901 * t9707 * t1443 * t28141 + t89 * t193 * t241 * t151347 * t258 / F::cast_from(3.0_f64);
    (t152164, t152179, t152218)
}
