//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2349/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2349<F: Float>(t118: F, t20800: F, t2576: F, t794: F, t210: F, t214: F, t41209: F, t41212: F, t41217: F, t59204: F, t59206: F, t59214: F, t59216: F, t59218: F, t59221: F, t59224: F, t67282: F, t787: F) -> F {
    let t68131 = t2576 * t118 * t794 * t20800;
    let t68141 = -F::cast_from(0.16666666666666666666e-2_f64) * t787 * t210 * t214 * t67282 + F::cast_from(0.8333333333333333333e-3_f64) * t68131 + t41209 + t41212 + F::cast_from(0.11666666666666666666e0_f64) * t59204 + F::cast_from(0.47499999999999999998e-1_f64) * t59206 + F::cast_from(0.24999999999999999999e-2_f64) * t59214 + F::cast_from(0.11666666666666666666e-1_f64) * t59216 - F::cast_from(0.15833333333333333333e-1_f64) * t59218 - F::cast_from(0.14999999999999999999e-1_f64) * t59221 + F::cast_from(0.49999999999999999998e-2_f64) * t59224 + F::cast_from(0.27777777777777777778e-3_f64) * t41217;
    t68141
}
