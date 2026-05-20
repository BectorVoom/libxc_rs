//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2203/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2203<F: Float>(t27833: F, t7901: F, t2014: F, t28020: F, t5542: F, t1450: F, t21969: F, t7237: F, t28167: F, t35669: F, t5627: F, t29996: F, t7235: F) -> (F, F, F, F, F) {
    let t109112 = F::new(6.0) * t27833 * t7901;
    let t109117 = F::new(2.0) * t2014 * t28020 * t5542;
    let t109118 = t1450 * t21969;
    let t109121 = F::new(3.0) * t2014 * t7237 * t109118;
    let t109124 = F::new(12.0) * t28167 * t35669 * t5627;
    let t109126 = F::new(2.0) * t7235 * t29996;
    (t109112, t109117, t109121, t109124, t109126)
}
