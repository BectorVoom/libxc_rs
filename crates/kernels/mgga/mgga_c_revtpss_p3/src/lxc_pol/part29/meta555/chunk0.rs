//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1896/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1896<F: Float>(t3917: F, t96576: F, t94701: F, t96204: F, t25878: F, t96242: F, t26359: F, t9303: F, t2118: F, t4153: F, t116: F, t26153: F) -> (F, F, F, F, F, F) {
    let t96577 = t96576 * t3917;
    let t96584 = F::cast_from(0.51727911450665971904e-3_f64) * t94701 * t96204;
    let t96588 = t25878 * t96242;
    let t96591 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t26359;
    let t96633 = t4153 * t2118;
    let t96640 = t116 * t26153;
    (t96577, t96584, t96588, t96591, t96633, t96640)
}
