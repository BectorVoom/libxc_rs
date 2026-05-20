//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 952/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk952<F: Float>(t32128: F, t8599: F, t2014: F, t10301: F, t8435: F, t8437: F) -> (F, F, F, F) {
    let t32129 = t8599 * t32128;
    let t32131 = F::new(2.0) * t2014 * t32129;
    let t32132 = t10301 * t8435;
    let t32135 = t8435 * t8437;
    (t32129, t32131, t32132, t32135)
}
