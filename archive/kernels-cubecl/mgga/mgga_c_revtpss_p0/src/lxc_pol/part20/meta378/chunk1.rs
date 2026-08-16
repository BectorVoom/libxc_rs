//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1371/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1371<F: Float>(t2430: F, t836: F, t10638: F, t125: F, t124: F, t2645: F, t14686: F, t14931: F, t4366: F, t2722: F, t10777: F, t10779: F, t2749: F) -> (F, F, F, F, F, F) {
    let t40560 = t2430 * t836;
    let t40569 = t125 * t10638;
    let t40578 = t124 * t2645;
    let t40581 = t14931 * t14686 * t40578 * t4366;
    let t40583 = t124 * t2722;
    let t40586 = t10777 * t10779 * t40583 * t2749;
    (t40560, t40569, t40578, t40581, t40583, t40586)
}
