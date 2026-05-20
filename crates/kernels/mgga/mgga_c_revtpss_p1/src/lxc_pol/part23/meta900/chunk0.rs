//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2861/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2861<F: Float>(t77047: F, t14330: F, t18575: F, t4186: F, t18259: F, t18306: F, t23210: F, t705: F, t707: F, t1522: F, t61122: F, t40205: F) -> (F, F, F, F, F, F) {
    let t77048 = F::cast_from(0.5848223622634646207e0_f64) * t77047;
    let t77051 = F::new(72.0) * t14330 * t18575 * t4186;
    let t77053 = F::new(36.0) * t18259 * t18306;
    let t77054 = t705 * t23210;
    let t77056 = F::new(4.0) * t77054 * t707;
    let t77058 = F::new(12.0) * t61122 * t1522;
    let t77059 = F::cast_from(0.35089341735807877242e1_f64) * t40205;
    (t77048, t77051, t77053, t77056, t77058, t77059)
}
