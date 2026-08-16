//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 546/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk546<F: Float>(t2258: F, t871: F, t2172: F, t2221: F, t2175: F, t2187: F, t2205: F, t2210: F, t2216: F, t2218: F, t2224: F, t2228: F, t2232: F) -> (F, F, F, F) {
    let t2259 = t2258 * t871;
    let t2264 = F::cast_from(0.68863333333333333333e0_f64) * t2172;
    let t2269 = F::cast_from(0.17365833333333333333e0_f64) * t2221;
    let t2273 = -F::cast_from(0.17648625e1_f64) * t2205 + F::cast_from(0.3529725e1_f64) * t2210 + t2264 - F::cast_from(0.103295e1_f64) * t2175 + F::cast_from(0.1549425e1_f64) * t2187 + F::cast_from(0.31558125e0_f64) * t2216 + F::cast_from(0.6311625e0_f64) * t2218 + t2269 - F::cast_from(0.41678e0_f64) * t2224 + F::cast_from(0.312585e0_f64) * t2228 + F::cast_from(0.312585e0_f64) * t2232;
    (t2259, t2264, t2269, t2273)
}
