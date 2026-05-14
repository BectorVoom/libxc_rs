//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1463/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1463<F: Float>(t10400: F, t23221: F, t23225: F, t23226: F, t23230: F, t23281: F, t23283: F, t4977: F, t6959: F, t8591: F, t8595: F, t8599: F, t9906: F, t10403: F, t23232: F, t23235: F, t23236: F, t23237: F, t23238: F, t4988: F, t8603: F, t8631: F, t8636: F, t8638: F, t9566: F, t9568: F) -> (F, F) {
    let t35288 = t9906 + t4977 + 0.97592231702715658576e-1 * t6959 - t23221 + t23281 + 0.59255020495841404221e-1 * t8591 + t23225 - t23226 - t23283 + t23230 + 36.0 * t8595 + t10400 + 9.0 * t8599;
    let t35295 = -3.0 * t8603 + 9.0 * t8631 + 3.0 * t9566 - t10403 - t4988 - t23232 + 18.0 * t9568 + 3.0 * t8636 + 6.0 * t8638 + t23235 - t23236 - t23237 - t23238;
    (t35288, t35295)
}
