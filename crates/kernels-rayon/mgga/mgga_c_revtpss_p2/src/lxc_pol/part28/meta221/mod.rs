//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1047;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta221(t1082: f64, t4757: f64, t1089: f64, t4905: f64, t1651: f64, t3291: f64, t4772: f64, t354: f64, t357: f64, t999: f64, t4781: f64, t3298: f64, t378: f64, t342: f64, t3154: f64, t3302: f64, t1043: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4961, t4964, t4967, t4970, t4975) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1047(t1082, t4757, t1089, t4905, t1651, t3291, t4772, t354, t357);
        let (t4976, t4977, t4980, t4981, t4982, t4983) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1048(t4975, t999, t4781, t3298, t378, t342, t3154, t3302, t1043);
    (t4961, t4964, t4967, t4970, t4975, t4976, t4977, t4980, t4981, t4982, t4983)
}
