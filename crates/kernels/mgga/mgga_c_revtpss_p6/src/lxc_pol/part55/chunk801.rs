//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 801/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk801<F: Float>(t552: F, t8591: F, t117: F, t8460: F, t572: F, t136: F, t8440: F) -> (F, F, F, F) {
    let t8592 = t8591 * t552;
    let t8614 = t117 * t8460;
    let t8616 = F::new(3.0) * t572 * t8614;
    let t8621 = t136 * t8440;
    (t8592, t8614, t8616, t8621)
}
