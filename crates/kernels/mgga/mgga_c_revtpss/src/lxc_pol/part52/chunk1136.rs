//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1136/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1136<F: Float>(t124: F, t561: F, t1353: F, t9818: F, t121174: F, t49068: F, t7301: F, t119971: F, t8705: F, t32265: F, t3974: F, t119967: F, t121173: F) -> (F, F, F, F, F, F, F) {
    let t121204 = t124 * t561;
    let t121206 = t9818 * t121204 * t1353;
    let t121207 = t121174 * t121206;
    let t121208 = F::cast_from(0.26773803678175077508e-3_f64) * t121207;
    let t121210 = t7301 * t49068;
    let t121211 = t119971 * t8705 * t121210;
    let t121227 = t32265 * t3974;
    let t121232 = t119967 * t121173;
    (t121204, t121206, t121208, t121210, t121211, t121227, t121232)
}
