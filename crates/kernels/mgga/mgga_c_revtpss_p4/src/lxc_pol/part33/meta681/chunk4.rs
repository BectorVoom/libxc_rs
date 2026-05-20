//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2225/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2225<F: Float>(t5883: F, t7583: F, t108129: F, t108681: F, t108685: F, t108687: F, t108691: F, t108693: F, t108712: F, t108716: F, t108718: F, t108721: F, t108723: F, t108725: F, t108727: F, t1310: F, t2163: F, t21814: F, t21882: F, t30724: F, t508: F, t5517: F, t5877: F, t7586: F, t7683: F, t8152: F) -> (F, F) {
    let t111708 = t7583 * t5883;
    let t111717 = -F::new(2.0) * t111708 * t508 - F::new(2.0) * t1310 * t30724 - t2163 * t21814 - F::new(2.0) * t21882 * t7586 - F::new(2.0) * t5517 * t8152 - t5877 * t7683 - t108129 + t108681 - t108685 + t108687 + t108691 + t108693 - t108712 - t108716 - t108718 - t108721 - t108723 - t108725 - t108727;
    (t111708, t111717)
}
