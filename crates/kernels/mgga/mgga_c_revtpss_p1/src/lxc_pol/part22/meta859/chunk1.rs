//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3008/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3008<F: Float>(t1544: F, t2645: F, t2722: F, t1558: F, t231: F, t40406: F, t685: F, t72: F, t826: F, t14869: F, t9775: F, t10899: F, t136: F, t216: F) -> (F, F, F, F, F) {
    let t50418 = t1544 * t2645;
    let t50423 = t1544 * t2722;
    let t50436 = t40406 * t826 * t1558 * t231 * t72 * t685;
    let t50443 = t9775 * t14869;
    let t50446 = t216 * t10899 * t136;
    (t50418, t50423, t50436, t50443, t50446)
}
