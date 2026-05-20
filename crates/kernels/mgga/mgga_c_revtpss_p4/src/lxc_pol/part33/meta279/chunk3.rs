//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1247/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1247<F: Float>(t1518: F, t7586: F, t7888: F, t7891: F, t7893: F, t8152: F, t118: F, t1502: F, t1519: F, t1843: F, t1911: F, t2127: F, t2163: F, t2165: F, t508: F, t569: F, t651: F, t7731: F, t7734: F, t7737: F, t7744: F, t7899: F, t7903: F, t7936: F, t7938: F, t8158: F, t8233: F) -> (F, F) {
    let t8237 = F::new(2.0) * t1518 * t7586 + t7888 + t7891 + t7893 + t8152;
    let t8240 = -t118 * t8233 - t1502 * t2163 - F::new(2.0) * t1519 * t7586 - t1843 * t2127 + t1911 * t2165 - t508 * t8152 + t569 * t8237 - F::new(2.0) * t651 * t8158 - t7731 - t7734 - t7737 - t7744 + t7899 + t7903 + t7936 - t7938;
    (t8237, t8240)
}
