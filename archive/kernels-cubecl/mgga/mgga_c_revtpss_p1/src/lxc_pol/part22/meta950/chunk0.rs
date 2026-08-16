//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3191/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3191<F: Float>(t17583: F, t3172: F, t3711: F, t127: F, t17693: F, t17695: F, t5268: F, t17708: F, t45779: F, t13089: F, t5391: F, t13085: F, t5381: F) -> (F, F, F, F, F) {
    let t59386 = t3711 * t3172 * t17583;
    let t59391 = t17693 * t127 * t5268 * t17695;
    let t59401 = t45779 * t17708;
    let t59404 = t5391 * t13089;
    let t59406 = t5381 * t13085;
    (t59386, t59391, t59401, t59404, t59406)
}
