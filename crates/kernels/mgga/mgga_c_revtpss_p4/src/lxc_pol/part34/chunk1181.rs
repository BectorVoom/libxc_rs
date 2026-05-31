//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1181/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1181<F: Float>(t18245: F, t1937: F, t30138: F, t4248: F, t7735: F, t1519: F, t1911: F, t2011: F, t28030: F, t29993: F, t29998: F, t30007: F, t30015: F, t30113: F, t30116: F, t30119: F, t30125: F, t30127: F, t30130: F, t30150: F, t569: F, t5887: F, t5921: F, t651: F, t6934: F, t6985: F, t7746: F, t7894: F) -> F {
    let t30154 = F::cast_from(2.0_f64) * t18245 * t1937;
    let t30156 = F::cast_from(4.0_f64) * t30138 * t1937;
    let t30158 = F::cast_from(4.0_f64) * t4248 * t7735;
    let t30159 = -F::cast_from(4.0_f64) * t1519 * t28030 + F::cast_from(2.0_f64) * t1911 * t7894 + t2011 * t6934 - F::cast_from(4.0_f64) * t30116 * t651 - F::cast_from(2.0_f64) * t30119 * t651 + t30150 * t569 - F::cast_from(4.0_f64) * t4248 * t7746 - F::cast_from(4.0_f64) * t5887 * t6985 - F::cast_from(2.0_f64) * t5921 * t6985 - t29993 - t29998 - t30007 + t30015 + t30113 - t30125 - t30127 - t30130 - t30154 - t30156 - t30158;
    t30159
}
