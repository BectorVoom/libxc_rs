//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1066/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1066<F: Float>(t121134: F, t121365: F, t32296: F, t531: F, t25081: F, t8763: F, t33553: F, t575: F, t1464: F, t8970: F, t136: F, t33362: F) -> (F, F, F, F, F, F) {
    let t121366 = t121365 * t121134;
    let t121441 = t531 * t32296;
    let t122820 = t8763 * t25081;
    let t124440 = t33553 * t575;
    let t124442 = t8970 * t1464;
    let t124455 = t33362 * t136;
    (t121366, t121441, t122820, t124440, t124442, t124455)
}
