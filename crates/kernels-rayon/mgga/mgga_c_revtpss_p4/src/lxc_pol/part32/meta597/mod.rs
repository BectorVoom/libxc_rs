//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1930;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta597(t2435: f64, t28448: f64, t28314: f64, t93364: f64, t103431: f64, t25375: f64, t212: f64, t28340: f64, t689: f64, t780: f64, t103182: f64, t93281: f64, t103421: f64, t7058: f64, t11064: f64, t8019: f64, t28993: f64, t571: f64, t2118: f64, t5789: f64, t1464: f64, t8113: f64, t1913: f64, t7560: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t103490, t103494, t103521, t103529, t103543) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1930(t2435, t28448, t28314, t93364, t103431, t25375, t212, t28340, t689, t780, t103182, t93281);
        let (t103547, t103586, t104062, t104071, t104073, t104077) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1931(t103421, t7058, t11064, t8019, t28993, t571, t2118, t5789, t1464, t8113, t1913, t7560);
    (t103490, t103494, t103521, t103529, t103543, t103547, t103586, t104062, t104071, t104073, t104077)
}
