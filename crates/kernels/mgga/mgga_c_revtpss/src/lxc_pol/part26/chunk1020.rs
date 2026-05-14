//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1020/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1020<F: Float>(t10208: F, t68: F, t10209: F, t2366: F, t665: F, t25826: F, t10254: F, t6998: F, t1450: F, t9628: F, t10426: F, t196: F, t197: F, t25081: F, t7234: F, t1464: F, t7541: F) -> (F, F, F, F, F, F, F) {
    let t94982 = t68 * t10208;
    let t94983 = t94982 * t10209;
    let t94985 = t665 * t2366;
    let t94986 = t25826 * t94985;
    let t94988 = t6998 * t10254;
    let t95002 = t1450 * t9628;
    let t95019 = t10426 * t196 * t197;
    let t95088 = t7234 * t25081;
    let t95182 = t7541 * t1464;
    (t94983, t94986, t94988, t95002, t95019, t95088, t95182)
}
