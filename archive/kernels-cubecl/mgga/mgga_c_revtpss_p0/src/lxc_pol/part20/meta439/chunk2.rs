//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1665/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1665<F: Float>(t1247: F, t13075: F, t3172: F, t1209: F, t13126: F, t17708: F, t127: F, t12988: F, t12989: F, t371: F, t1203: F, t12626: F) -> (F, F, F, F) {
    let t45352 = t1247 * t3172 * t13075;
    let t45371 = t1209 * t13126 * t17708;
    let t45382 = t12988 * t371 * t127 * t12989;
    let t45384 = t1203 * t12626;
    (t45352, t45371, t45382, t45384)
}
