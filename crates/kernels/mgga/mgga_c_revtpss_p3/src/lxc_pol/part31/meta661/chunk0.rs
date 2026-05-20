//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2239/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2239<F: Float>(t1353: F, t30122: F, t28167: F, t8717: F, t2014: F, t25190: F, t29494: F, t27833: F, t7901: F, t28020: F, t5542: F, t1450: F, t21969: F) -> (F, F, F, F, F) {
    let t109104 = t30122 * t1353;
    let t109107 = F::new(12.0) * t28167 * t8717 * t109104;
    let t109110 = F::new(3.0) * t2014 * t25190 * t29494;
    let t109112 = F::new(6.0) * t27833 * t7901;
    let t109117 = F::new(2.0) * t2014 * t28020 * t5542;
    let t109118 = t1450 * t21969;
    (t109107, t109110, t109112, t109117, t109118)
}
