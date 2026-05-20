//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3384/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3384<F: Float>(t11294: F, t19250: F, t15474: F, t2924: F, t4635: F, t63212: F, t63214: F, t63216: F, t63218: F, t63220: F, t63222: F, t63224: F, t63226: F, t63228: F, t63579: F, t63581: F) -> (F, F, F) {
    let t63583 = F::cast_from(0.64327917994770140268e2_f64) * t11294 * t19250;
    let t63586 = F::cast_from(0.32163958997385070134e2_f64) * t2924 * t4635 * t15474;
    let t63587 = -t63212 + t63214 - t63216 + t63218 + t63220 - t63222 - t63224 + t63226 + t63228 + t63579 + t63581 + t63583 + t63586;
    (t63583, t63586, t63587)
}
