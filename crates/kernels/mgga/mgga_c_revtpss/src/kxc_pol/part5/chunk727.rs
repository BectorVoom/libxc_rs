//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 727/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk727<F: Float>(t1134: F, t5071: F, t3358: F, t3394: F, t5044: F, t5049: F, t5054: F, t5058: F, t1132: F, t1723: F, t3407: F, t1139: F, t1729: F, t698: F) -> (F, F, F, F, F, F, F) {
    let t5072 = t5071 * t1134;
    let t5079 = t3394 - t3358 / 9.0 - t5044 / 9.0 - 2.0 / 9.0 * t5049 + 2.0 / 3.0 * t5054 + t5058 / 3.0;
    let t5080 = t1132 * t5079;
    let t5087 = t3407 * t1723;
    let t5088 = t5087 * t1134;
    let t5090 = t1139 * t5079;
    let t5093 = t698 * t1729;
    (t5072, t5079, t5080, t5087, t5088, t5090, t5093)
}
