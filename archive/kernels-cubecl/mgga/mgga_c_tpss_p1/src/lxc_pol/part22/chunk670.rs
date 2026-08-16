//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 670/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk670<F: Float>(t1165: F, t2054: F, t2056: F, t2061: F, t2105: F, t645: F, t93: F, t1183: F, t177: F, t737: F, t1193: F, t2206: F) -> (F, F, F, F, F) {
    let t3174 = F::cast_from(2.0_f64) * t1165 * t2105 + F::cast_from(4.0_f64) * t2056 * t645 + F::cast_from(2.0_f64) * t2061 * t93 + t2054;
    let t3178 = t1183 * t177;
    let t3179 = t3178 * t737;
    let t3180 = F::cast_from(0.11696447245269292414e1_f64) * t3179;
    let t3182 = F::cast_from(0.5848223622634646207e0_f64) * t1193 * t2206;
    (t3174, t3178, t3179, t3180, t3182)
}
