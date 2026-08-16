//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 420/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk420(t2097: f64, t225: f64, t561: f64, t545: f64, t2028: f64, t2027: f64, t213: f64) -> (f64, f64, f64, f64) {
    let t2098 = t2097 * t225;
    let t2099 = t2098 * t561;
    let t2102 = t545 * t2097;
    let t2103 = t2028 * t2102;
    let t2106 = 0.65854491829355115987e0_f64 * t213 * t2099 - 0.4336814094102599731e0_f64 * t2027 * t2103;
    (t2098, t2102, t2103, t2106)
}
