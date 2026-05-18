//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 825/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk825<F: Float>(t2055: F, t7586: F, t8564: F, t8689: F, t8694: F, t8886: F, t2052: F, t2056: F, t2089: F, t2108: F, t2127: F, t2163: F, t508: F, t569: F, t651: F, t8463: F, t8630: F, t8636: F, t8643: F, t8687: F, t8699: F, t8716: F, t8719: F, t8764: F, t8892: F) -> (F, F) {
    let t8897 = F::new(2.0) * t2055 * t7586 + t8564 + t8689 + t8694 + t8886;
    let t8900 = -t2052 * t2163 - F::new(2.0) * t2056 * t7586 - t2089 * t2127 + t2108 * t8764 - t508 * t8886 + t569 * t8897 - F::new(2.0) * t651 * t8892 - t8463 - t8630 - t8636 - t8643 - t8687 + t8699 + t8716 - t8719;
    (t8897, t8900)
}
