//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 779/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk779<F: Float>(t2148: F, t6541: F, t2147: F, t2155: F, t5169: F, t2132: F, t2183: F, t2262: F, t797: F, t296: F, t297: F, t306: F, t307: F) -> (F, F, F, F, F, F) {
    let t6542 = t2148 * t6541;
    let t6543 = t2147 * t6542;
    let t6545 = t2155 * t5169;
    let t6583 = t2183 * t2132;
    let t6599 = t2262 * t797;
    let t6621 = F::new(1.0) / t297 / t296;
    let t6635 = F::new(1.0) / t307 / t306;
    (t6543, t6545, t6583, t6599, t6621, t6635)
}
