//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 769/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk769<F: Float>(t3: F, t8970: F, t573: F, t8613: F, t8616: F, t8773: F, t197: F, t532: F, t1450: F, t2033: F, t4146: F, t565: F) -> (F, F, F, F, F, F) {
    let t8971 = t3 * t8970;
    let t8975 = param_d * t8970;
    let t8978 = t573 * t8975 + t8613 + t8616 + F::new(6.0) * t8773;
    let t8995 = t197 * t532;
    let t8996 = t2033 * t1450;
    let t9593 = F::new(1.0) / t4146 / t565;
    (t8971, t8975, t8978, t8995, t8996, t9593)
}
