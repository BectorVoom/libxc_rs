//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 689/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk689<F: Float>(t552: F, t8591: F, t8576: F, t8579: F, t8580: F, t8583: F, t8586: F) -> (F,) {
    let t8592 = t8591 * t552;
    let t8594 = 0.28234466758480466999e-3 * t8576 - 0.8673628188205199462e0 * t8579 * t8580 + 0.57119737665102352616e0 * t8583 * t8586 - 0.1859366460452550541e-3 * t8592;
    (t8594,)
}
