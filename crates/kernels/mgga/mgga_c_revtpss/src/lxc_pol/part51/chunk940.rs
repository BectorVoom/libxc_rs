//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 940/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk940<F: Float>(t120986: F, t32705: F, t239: F, t8583: F, t8589: F, t9990: F, t1405: F, t32272: F, t32269: F, t3974: F, t120981: F, t32710: F, t1389: F, t31752: F, t32192: F, t32282: F, t8584: F) -> (F, F, F, F, F, F, F, F) {
    let t120987 = t32705 * t120986;
    let t120991 = t8583 * t8589 * t9990 * t239;
    let t120994 = t32272 * t1405;
    let t120996 = t32269 * t3974;
    let t120997 = 0.3526350471130277186e-3 * t120996;
    let t121000 = t32269 * t120981;
    let t121003 = t32710 * t120986;
    let t121011 = t31752 * t32192 * t1389;
    let t121018 = t8583 * t8584 * t32282;
    (t120987, t120991, t120994, t120997, t121000, t121003, t121011, t121018)
}
