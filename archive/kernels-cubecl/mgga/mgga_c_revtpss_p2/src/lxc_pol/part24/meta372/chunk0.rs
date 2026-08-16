//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1262/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1262<F: Float>(t1277: F, t24514: F, t1774: F, t3737: F, t6702: F, t1828: F, t13182: F, t13100: F, t24228: F, t247: F, t1794: F, t6628: F) -> (F, F, F, F, F, F) {
    let t24515 = t1277 * t24514;
    let t24519 = t3737 * t1774 * t6702;
    let t24524 = t6702 * t1828;
    let t24525 = t13182 * t24524;
    let t24535 = t247 * t13100 * t24228;
    let t24543 = t6628 * t1794;
    (t24515, t24519, t24524, t24525, t24535, t24543)
}
