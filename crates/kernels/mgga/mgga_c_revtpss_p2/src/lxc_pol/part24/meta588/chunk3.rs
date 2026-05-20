//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1839/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1839<F: Float>(t1343: F, t1450: F, t1868: F, t198: F, t21937: F, t39419: F, t39422: F, t4139: F, t46292: F, t46297: F, t46303: F, t532: F, t5536: F, t6816: F, t6836: F, t86731: F, t86839: F, t91826: F, t91952: F, t91953: F, t91954: F, t91955: F, t92229: F, t92248: F, t92267: F, t92434: F) -> F {
    let t92446 = F::new(36.0) * t198 * t86839 * t6816 + t46292 - t46297 + F::new(3.0) * t198 * t1343 * t91826 + t198 * t532 * (t92229 + t92248 + t92267 + t92434) * t1450 - t39419 - t39422 + t46303 + t91952 - t91953 + t91954 + t91955 + F::new(36.0) * t5536 * t21937 * t6836 + F::new(12.0) * t4139 * t86731 * t1868;
    t92446
}
