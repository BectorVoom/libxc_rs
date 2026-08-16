//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3185/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3185<F: Float>(t12866: F, t17514: F, t56756: F, t12916: F, t17723: F, t3718: F, t12832: F, t17617: F, t12851: F, t1778: F, t17429: F, t17789: F) -> (F, F, F, F, F) {
    let t59078 = t12866 * t56756 * t17514;
    let t59094 = t3718 * t12916 * t17723;
    let t59142 = t12832 * t17617;
    let t59144 = t1778 * t12851;
    let t59146 = t17429 * t17789;
    (t59078, t59094, t59142, t59144, t59146)
}
