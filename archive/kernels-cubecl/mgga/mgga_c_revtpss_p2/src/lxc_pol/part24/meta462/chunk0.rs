//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1434/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1434<F: Float>(t12854: F, t21013: F, t12808: F, t13036: F, t13039: F, t57403: F, t3597: F, t12469: F, t1737: F, t1729: F, t9303: F, t12552: F, t1749: F) -> (F, F, F, F, F, F, F) {
    let t57707 = t12854 * t21013;
    let t57710 = t12808 * t21013;
    let t57759 = t13036 * t13039 * t57403;
    let t57763 = t13036 * t3597 * t57403;
    let t58005 = t1737 * t12469;
    let t58153 = t9303 * t1729;
    let t58247 = t1749 * t12552;
    (t57707, t57710, t57759, t57763, t58005, t58153, t58247)
}
