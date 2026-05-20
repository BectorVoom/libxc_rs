//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1832/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1832<F: Float>(t3133: F, t73: F, t3153: F, t2258: F, t3094: F, t3182: F, t828: F) -> (F, F, F, F) {
    let t11678 = t3133 * t73;
    let t11687 = t3133 * t3153;
    let t11696 = t3094 * t2258;
    let t11703 = t828 * t3182;
    (t11678, t11687, t11696, t11703)
}
