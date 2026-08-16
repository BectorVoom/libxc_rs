//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 834/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk834(t8973: f64, t8981: f64, t7840: f64, t7845: f64, t7848: f64, t7865: f64, t8291: f64, t8292: f64, t8294: f64, t8963: f64, t8967: f64, t8971: f64, t8975: f64, t8979: f64, t8983: f64) -> f64 {
    let t9356 = 0.64311027177104605458e-2_f64 * t8973;
    let t9359 = 0.94344276868812456204e-2_f64 * t8981;
    let t9363 = 0.62896184579208304138e-3_f64 * t8963 - 0.94344276868812456207e-3_f64 * t8967 + 0.31448092289604152069e-3_f64 * t8971 + t9356 - 0.56606566121287473724e-2_f64 * t8975 - 0.42874018118069736972e-3_f64 * t8979 - t9359 + 0.25724410870841842183e-2_f64 * t8983 + 0.31448092289604152069e-3_f64 * t7840 + 0.20965394859736101379e-3_f64 * t7845 - t7848 + t8291 + t8292 + t8294 - t7865;
    t9363
}
