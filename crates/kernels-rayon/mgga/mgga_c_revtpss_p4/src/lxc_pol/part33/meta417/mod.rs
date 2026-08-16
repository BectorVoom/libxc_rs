//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1486;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta417(t5941: f64, t72: f64, t757: f64, t10569: f64, t4186: f64, t4402: f64, t4401: f64, t177: f64, t762: f64, t10579: f64, t14386: f64, t1522: f64, t10566: f64, t10568: f64, t10577: f64, t10582: f64, t10584: f64, t10586: f64, t9514: f64, t9517: f64, t9521: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t18557, t18558, t18561, t18564, t18565, t18567) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1486(t5941, t72, t757, t10569, t4186, t4402, t4401, t177, t762, t10579, t14386, t1522);
        let t18568 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1487(t10566, t10568, t10577, t10582, t10584, t10586, t18557, t18558, t18561, t18564, t18565, t18567, t9514, t9517, t9521);
    (t18557, t18558, t18561, t18564, t18565, t18567, t18568)
}
