//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1272/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1272<F: Float>(t225: F, t24698: F, t480: F, t1774: F, t6622: F, t1250: F, t3720: F, t6587: F) -> (F, F, F, F, F, F) {
    let t24699 = t24698 * t225;
    let t24700 = t24699 * t480;
    let t24704 = t1774 * t6622;
    let t24705 = t24704 * t1250;
    let t24706 = t3720 * t24705;
    let t24713 = t1774 * t6587;
    (t24699, t24700, t24704, t24705, t24706, t24713)
}
