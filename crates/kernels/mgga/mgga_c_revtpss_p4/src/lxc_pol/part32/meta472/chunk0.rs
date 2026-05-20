//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1701/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1701<F: Float>(t25253: F, t25275: F, t25283: F, t122: F, t2061: F, t72: F, t25412: F, t25411: F, t2466: F, t25387: F, t2062: F, t867: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26462 = F::cast_from(0.30488190661738479625e-3_f64) * t25253;
    let t26468 = F::new(35.0) / F::new(216.0) * t25275;
    let t26471 = F::cast_from(0.10164000561857065645e-4_f64) * t25283;
    let t26481 = t2061 * t72 * t122;
    let t26482 = t26481 * t25412;
    let t26483 = t25411 * t26482;
    let t26485 = t26481 * t2466;
    let t26486 = t25387 * t26485;
    let t26496 = t2062 * t867;
    (t26462, t26468, t26471, t26481, t26482, t26483, t26485, t26486, t26496)
}
