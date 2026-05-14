//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 859/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk859<F: Float>(t1252: F, t1797: F, t26873: F, t26880: F, t29010: F, t29020: F, t29023: F, t29027: F, t29052: F, t29079: F, t29107: F, t5270: F, t5279: F, t5287: F, t5299: F, t5304: F, t7618: F, t7624: F) -> (F,) {
    let t29109 = 0.28582678745379824648e-3 * t26880 * t5299 - 0.57165357490759649296e-3 * t7624 * t5270 + 0.42874018118069736972e-3 * t29010 * t1252 + 0.28582678745379824648e-3 * t26880 * t5279 + 0.42874018118069736972e-3 * t26873 * t1797 + 0.42874018118069736972e-3 * t7618 * t5287 - 0.22866142996303859718e-2 * t29020 * t1252 + 0.28582678745379824648e-3 * t29023 + 0.47637797908966374413e-3 * t7624 * t5304 - t29027 / 108.0 + t29052 + t29079 + t29107;
    (t29109,)
}
