//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1083/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1083<F: Float>(t24634: F, t371: F, t372: F, t24610: F, t5302: F, t1042: F, t23842: F, t1774: F, t5825: F, t5296: F, t24244: F, t5308: F) -> (F, F, F, F, F) {
    let t24636 = t371 * t372 * t24634;
    let t24639 = t5302 * t24610;
    let t24640 = t1042 * t24639;
    let t24643 = t5302 * t23842;
    let t24644 = t1042 * t24643;
    let t24647 = t5825 * t1774;
    let t24648 = t5296 * t24647;
    let t24649 = t1042 * t24648;
    let t24652 = t5308 * t24244;
    (t24636, t24640, t24644, t24649, t24652)
}
