//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2536/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2536<F: Float>(t51973: F, t52035: F, t52037: F, t2852: F, t373: F, t2439: F, t4628: F, t1606: F, t9303: F, t2923: F, t4587: F, t11384: F, t1596: F) -> (F, F, F, F, F, F, F, F, F) {
    let t52082 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t51973;
    let t52091 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t52035;
    let t52092 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t52037;
    let t52110 = t373 * t2852;
    let t52126 = t2439 * t4628;
    let t52127 = F::cast_from(0.27595e0_f64) * t52126;
    let t52128 = t9303 * t1606;
    let t52219 = t4587 * t2923;
    let t52224 = t1596 * t11384;
    (t52082, t52091, t52092, t52110, t52126, t52127, t52128, t52219, t52224)
}
