//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1059/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1059<F: Float>(t121202: F, t124: F, t561: F, t1353: F, t9818: F, t121174: F, t49068: F, t7301: F, t119971: F, t8705: F, t121197: F, t32244: F) -> (F, F, F, F, F, F, F) {
    let t121203 = F::new(0.74664478761315012733e-2) * t121202;
    let t121204 = t124 * t561;
    let t121206 = t9818 * t121204 * t1353;
    let t121207 = t121174 * t121206;
    let t121210 = t7301 * t49068;
    let t121211 = t119971 * t8705 * t121210;
    let t121212 = F::new(0.23511941766261123138e-4) * t121211;
    let t121214 = F::new(0.33852964522850660984e-1) * t32244 * t121197;
    (t121203, t121204, t121206, t121207, t121210, t121212, t121214)
}
