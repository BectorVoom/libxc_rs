//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1130/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1130<F: Float>(t1405: F, t32272: F, t32269: F, t3974: F, t120981: F, t120986: F, t32710: F, t1389: F, t31752: F, t32192: F, t32282: F, t8583: F, t8584: F) -> (F, F, F, F, F, F) {
    let t120994 = t32272 * t1405;
    let t120995 = F::cast_from(0.17354086964223805049e-2_f64) * t120994;
    let t120996 = t32269 * t3974;
    let t121000 = t32269 * t120981;
    let t121003 = t32710 * t120986;
    let t121004 = F::cast_from(0.13223814266738539448e-3_f64) * t121003;
    let t121011 = t31752 * t32192 * t1389;
    let t121018 = t8583 * t8584 * t32282;
    (t120995, t120996, t121000, t121004, t121011, t121018)
}
