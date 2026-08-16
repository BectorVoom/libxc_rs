//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 959/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk959<F: Float>(t9959: F, t9966: F, t2345: F, t4438: F, t177: F, t4377: F, t737: F, t10022: F, t10120: F, t774: F, t1232: F, t1625: F) -> (F, F, F, F, F, F, F) {
    let t12754 = F::cast_from(12.0_f64) * t9959;
    let t12757 = F::cast_from(80.0_f64) * t9966;
    let t12758 = t4438 * t2345;
    let t12767 = t4377 * t177;
    let t12769 = F::cast_from(0.11696447245269292414e1_f64) * t12767 * t737;
    let t12780 = F::cast_from(48.0_f64) * t10022;
    let t12816 = t10120 * t774;
    let t12817 = t1625 * t1232;
    (t12754, t12757, t12758, t12769, t12780, t12816, t12817)
}
